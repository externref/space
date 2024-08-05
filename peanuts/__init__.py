import sys

from peanuts.session import Session


def main() -> None:
    session = Session(sys.argv[1])
    session.run_session()
