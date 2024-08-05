import sys

from peanuts.session import Session

session = Session(sys.argv[1])
session.run_session()
