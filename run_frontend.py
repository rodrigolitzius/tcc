import os
import signal
import subprocess
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer

PORT = 8000
HOST = "0.0.0.0"


def _kill_process_on_port(port: int) -> None:

    try:
        cmd_lsof = "lsof -t -i:{} || true".format(port)
        pids = subprocess.check_output(["sh", "-c", cmd_lsof], stderr=subprocess.DEVNULL, text=True)
        pids = [p for p in pids.strip().splitlines() if p.strip()]
        for pid in pids:
            try:
                os.kill(int(pid), signal.SIGKILL)
            except ProcessLookupError:
                pass

        subprocess.call(["sh", "-c", "sleep 0.2"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


        cmd_fuser = "fuser -k {}/tcp || true".format(port)
        subprocess.call(["sh", "-c", cmd_fuser], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

        subprocess.call(["sh", "-c", "sleep 0.2"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    except Exception:
        pass




if __name__ == "__main__":
    import sys

    cmd = (sys.argv[1].lower() if len(sys.argv) > 1 else "run").strip()

    if cmd in ("run", "start"):
        _kill_process_on_port(PORT)
        server = ThreadingHTTPServer((HOST, PORT), SimpleHTTPRequestHandler)
        print(f"Serving frontend at http://localhost:{PORT}")
        server.serve_forever()
    elif cmd in ("kill", "stop"):
        _kill_process_on_port(PORT)
        print(f"Stopped anything using port {PORT} (if any).")
    else:
        print("Usage: python3 run_frontend.py [run|kill]")
        sys.exit(1)


