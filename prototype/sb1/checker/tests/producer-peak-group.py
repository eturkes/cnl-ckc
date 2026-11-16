import argparse,json,os,select,signal,subprocess,time
from pathlib import Path
p=argparse.ArgumentParser()
for key in ['server','root','out']: p.add_argument('--'+key,type=Path,required=True)
p.add_argument('--plant-detach',action='store_true')
a=p.parse_args(); a.out.mkdir(parents=True,exist_ok=False)
fake=a.out/'fake-swipl'
fake.write_text('#!/usr/bin/env python3\nimport os,signal\n'+('os.setsid()\n' if a.plant_detach else '')+'while True: signal.pause()\n'); fake.chmod(0o700)
instance=a.root/'rust/target/bench/sb1-frozen/F1b-g001-s0'
argv=[str(a.server),'serve','--provider','static','--ta-receipts','none','--swipl',str(fake),'--root',str(a.root),str(instance/'composition.tsv'),str(a.out/'arm')]
(a.out/'argv.json').write_text(json.dumps(argv,indent=2)+'\n')

def line(child):
 end=time.monotonic()+180; text=bytearray()
 while not text.endswith(b'\n'):
  left=end-time.monotonic(); assert left>0 and select.select([child.stdout],[],[],left)[0]
  c=os.read(child.stdout.fileno(),1); assert c; text+=c
 return text.decode().rstrip('\n')

def dead(pid):
 try: return Path(f'/proc/{pid}/stat').read_text().rsplit(')',1)[1].split()[0]=='Z'
 except FileNotFoundError: return True

owned=[]
with (a.out/'stderr').open('wb') as stderr:
 child=subprocess.Popen(argv,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=stderr,start_new_session=True,bufsize=0)
 try:
  assert line(child)=='ready'
  startup=dict(row.split('\t',1) for row in (a.out/'arm/producer-launcher.tsv').read_text().splitlines())
  launcher=int(startup['pid']); owned.append(launcher)
  call=a.out/'call'; query=instance/'queries/q-one.pl'
  child.stdin.write(f'1\t{query}\t{call}\n'.encode()); child.stdin.flush()
  end=time.monotonic()+20; producers=[]
  while not producers:
   assert time.monotonic()<end
   producers=list(map(int,Path(f'/proc/{launcher}/task/{launcher}/children').read_text().split()))
   if not producers: select.select([],[],[],0.001)
  assert len(producers)==1; producer=producers[0]; owned.append(producer)
  if a.plant_detach:
   end=time.monotonic()+10
   while os.getpgid(producer)==child.pid:
    assert time.monotonic()<end
    select.select([],[],[],0.001)
  before={str(pid):os.getpgid(pid) for pid in [child.pid,launcher,producer]}
  (a.out/'before.json').write_text(json.dumps(before,indent=2)+'\n')
  assert set(before.values())=={child.pid}, 'group-control: detached producer/launcher'
  os.killpg(child.pid,signal.SIGKILL); rc=child.wait(timeout=10)
  end=time.monotonic()+10
  while not all(dead(pid) for pid in owned):
   assert time.monotonic()<end
   select.select([],[],[],0.001)
  report=dict(server_rc=rc,pids=before,all_terminated=True,producer_report_present=(call/'producer-report.tsv').exists(),launcher_terminal_present=(a.out/'arm/launcher-terminal.tsv').exists())
  assert rc==-signal.SIGKILL and not report['producer_report_present'] and not report['launcher_terminal_present']
  (a.out/'result.json').write_text(json.dumps(report,indent=2)+'\n'); print(json.dumps(report))
 finally:
  if child.poll() is None:
   os.killpg(child.pid,signal.SIGKILL); child.wait()
  for pid in owned:
   if not dead(pid):
    try: os.kill(pid,signal.SIGKILL)
    except ProcessLookupError: pass
  child.stdout.close(); child.stdin.close()
