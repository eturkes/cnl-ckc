"""Pre-load persistent wait4 spawner; inherited group, identity-bound producer reports.
EOF/error → atomic self-only terminal record → os._exit, with no interpreter teardown.
Request: LE u64 sequence + LE u32 count, each field LE u32 length + bytes.
Fields = call identity, stdout path, stderr path, executable, argv….
"""
import os
import resource
import struct
import subprocess
import sys


def send(*fields):
    print('\t'.join(map(str, fields)), flush=True)


def start_ticks(pid):
    with open(f'/proc/{pid}/stat', 'rb') as stream:
        return int(stream.read().rsplit(b')', 1)[1].split()[19])


def hwm(pid):
    with open(f'/proc/{pid}/status', 'rb') as stream:
        for line in stream:
            if line.startswith(b'VmHWM:'):
                return int(line.split()[1]) * 1024
    return 0  # Zombie: no observation; the positive raw wait4 term remains required.


def exact(count):
    data = bytearray()
    while len(data) < count:
        part = sys.stdin.buffer.read(count - len(data))
        if not part:
            raise EOFError('partial launcher request')
        data.extend(part)
    return bytes(data)


_launcher_pid = os.getpid()
_launcher_ticks = start_ticks(_launcher_pid)
_terminal = os.fsencode(sys.argv[1]) if len(sys.argv) == 2 else None
_terminal_tmp = _terminal + b'.' + str(_launcher_pid).encode() + b'.tmp' if _terminal else None
_terminal_fd = None


def terminal_exit(code):
    if _terminal_fd is None:
        os._exit(code)
    try:
        report_pid = _launcher_pid
        report_ticks = _launcher_ticks
        kind = 'normal' if code == 0 else 'error'
        raw = int(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss) * 1024
        own = hwm(_launcher_pid)
        payload = (f'format\tsb1-launcher-terminal 1\npid\t{report_pid}\nstart_ticks\t{report_ticks}\n'
                   f'resource_scope\tself\nru_maxrss_bytes\t{raw}\nvmhwm_bytes\t{own}\n'
                   f'exit_kind\t{kind}\nexit_code\t{code}\n').encode()
        while payload:
            count = os.write(_terminal_fd, payload)
            payload = payload[count:]
        os.fsync(_terminal_fd)
        os.close(_terminal_fd)
        # Paths/buffers + all formatting precede publication. The next call never returns.
        os.replace(_terminal_tmp, _terminal)
        os._exit(code)
    except BaseException:
        os._exit(2)  # No valid published record → parent marks this term unbounded.


def serve():
    send('ready', 1, _launcher_pid, os.getpgrp(), _launcher_ticks, hwm(_launcher_pid),
         '.'.join(map(str, sys.version_info[:3])))
    previous = 0
    while True:
        first = sys.stdin.buffer.read(1)
        if not first:
            return
        sequence = struct.unpack('<Q', first + exact(7))[0]
        count = struct.unpack('<I', exact(4))[0]
        if sequence <= previous or not 4 <= count <= 256:
            raise ValueError('launcher request sequence/count')
        previous = sequence
        fields = []
        for _ in range(count):
            size = struct.unpack('<I', exact(4))[0]
            if size > 1024 * 1024:
                raise ValueError('launcher request field length')
            fields.append(exact(size))
        identity = fields[0].decode('ascii')
        if len(identity) != 64 or any(c not in '0123456789abcdef' for c in identity):
            raise ValueError('launcher call identity')
        child = None
        reaped = False
        try:
            with open(fields[1], 'wb') as stdout, open(fields[2], 'wb') as stderr:
                child = subprocess.Popen(fields[3:], stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr)
            pid = child.pid
            ticks = start_ticks(pid)
            observed = hwm(pid)
            send('started', sequence, identity, pid, ticks)
            ack = sys.stdin.buffer.readline(4097).decode('ascii').rstrip('\n').split('\t')
            if len(ack) != 6 or ack[:5] != ['ack', str(sequence), identity, str(pid), str(ticks)]:
                raise ValueError('launcher acknowledgement')
            observed = max(observed, int(ack[5]), hwm(pid))
            while True:
                try:
                    pid, status, usage = os.wait4(child.pid, 0)
                    break
                except InterruptedError:
                    continue
            reaped = True
            child.returncode = os.waitstatus_to_exitcode(status)
            wait4_peak = int(usage.ru_maxrss) * 1024
            cpu_us = round(usage.ru_utime * 1_000_000) + round(usage.ru_stime * 1_000_000)
            kind = 'exit' if os.WIFEXITED(status) else 'signal'
            if pid != child.pid or wait4_peak <= 0:
                raise ValueError(f'launcher wait4 identity/peak pid={pid} expected={child.pid} raw={wait4_peak} observed={observed}')
            peak = max(wait4_peak, observed)
            send('exited', sequence, identity, pid, ticks, kind, status, wait4_peak, peak, cpu_us, observed)
            send('complete', sequence, identity)
        except (OSError, ValueError) as error:
            send('failed', sequence, str(error).encode().hex())
            raise
        finally:
            if child is not None and not reaped:
                try:
                    child.kill()
                except ProcessLookupError:
                    pass
                child.wait()


_code = 0
try:
    if _terminal is not None:
        _terminal_fd = os.open(_terminal_tmp, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    serve()
except BaseException:
    _code = 2
terminal_exit(_code)
