"""D7 fixed1000-call qualification: same-production-helper raw/observed distribution; no timing grade."""
import argparse,hashlib,importlib.util,json,os,struct
from pathlib import Path
p=argparse.ArgumentParser(); sub=p.add_subparsers(dest='mode',required=True)
r=sub.add_parser('record')
for key in ['script','driver','out']: r.add_argument('--'+key,type=Path,required=True)
g=sub.add_parser('grade'); g.add_argument('out',type=Path); g.add_argument('--plant',choices=['missing','identity','raw-only'])
a=p.parse_args()

def grade(out,plant=None):
 rows=[json.loads(s) for s in (out/'calls.jsonl').read_text().splitlines()]
 if plant=='missing': rows.pop()
 if plant=='identity': rows[0]['call_identity']='0'*64
 if plant=='raw-only':
  target=next((row for row in rows if row['observed_hwm_bytes']>row['producer_wait4_maxrss_bytes']),rows[0])
  target['observed_hwm_bytes']=max(target['observed_hwm_bytes'],target['producer_wait4_maxrss_bytes']+1024)
  target['peak_bytes']=target['producer_wait4_maxrss_bytes']
 assert len(rows)==1000, 'D7 complete count'
 for n,row in enumerate(rows,1):
  expected=hashlib.sha256(b'sb1-call\0'+struct.pack('<Q',n)+os.fsencode(out)).hexdigest()
  assert row['call_identity']==expected and row['raw_status']==0 and row['kind']=='exit', 'D7 identity/status'
  assert 0<row['producer_wait4_maxrss_bytes'] and row['peak_bytes']==max(row['producer_wait4_maxrss_bytes'],row['observed_hwm_bytes'])<=64<<20, 'D7 conservative maximum'
 gaps=[max(0,row['observed_hwm_bytes']-row['producer_wait4_maxrss_bytes']) for row in rows]
 result=dict(calls=len(rows),failures=0,observed_above_wait4=sum(gap>0 for gap in gaps),max_gap_bytes=max(gaps),max_charged_bytes=max(row['peak_bytes'] for row in rows),raw_wait4_range=[min(row['producer_wait4_maxrss_bytes'] for row in rows),max(row['producer_wait4_maxrss_bytes'] for row in rows)])
 launcher=json.loads((out/'launcher.json').read_text()); terminal=dict(line.split('\t',1) for line in (out/'launcher-terminal.tsv').read_text().splitlines())
 assert int(terminal['pid'])==launcher['pid'] and int(terminal['start_ticks'])==launcher['start_ticks']
 assert terminal['resource_scope']=='self' and terminal['exit_kind']=='normal' and terminal['exit_code']=='0'
 own=max(int(terminal['ru_maxrss_bytes']),int(terminal['vmhwm_bytes']))
 assert launcher['after_hwm_bytes']<=own<=64<<20
 result['launcher_terminal_bound_bytes']=own
 if not plant: (out/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
 print(json.dumps(result,indent=2))

if a.mode=='record':
 a.out.mkdir(parents=True,exist_ok=False)
 spec=importlib.util.spec_from_file_location('control',a.driver); mod=importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
 launcher=mod.Launcher(a.script,a.out)
 big=bytearray(2<<30)
 for at in range(0,len(big),4096): big[at]=1
 del big
 try:
  with (a.out/'calls.jsonl').open('w') as stream:
   for n in range(1000):
    row=launcher.run(['true']); stream.write(json.dumps(row)+'\n'); stream.flush()
 finally:
  if launcher.process.poll() is None:
   terminal_state=launcher.close(); (a.out/'launcher.json').write_text(json.dumps(terminal_state,indent=2)+'\n')
 grade(a.out)
else: grade(a.out,a.plant)
