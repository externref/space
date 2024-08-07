import sys

from peanuts.session import Session


def main() -> None:
    session = Session(sys.argv[1])
    if len(sys.argv) > 2 and sys.argv[2] in ["--compile", "-c"]:
        assert len(sys.argv) == 4
        session.run_command(sys.argv[3].split()[0], sys.argv[3].split()[1:])
        return
    session.run_session()
