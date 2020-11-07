from cryptoquip import run

import sys
import traceback

def main():
    try: 
        run.run()
    except (KeyboardInterrupt, SystemExit):
        raise
    except:
        print('Error in Cryptoquip downloader', file=sys.stderr)
        traceback.print_exc()
        input('\n(Press ENTER to exit)\n> ')


if __name__ == "__main__":
    main()