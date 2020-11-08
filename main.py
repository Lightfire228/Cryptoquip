from cryptoquip import run, utils, dirs

import sys
import traceback

def main():
    try: 
        run.run()
    except (KeyboardInterrupt, SystemExit):
        raise
    except:

        if not dirs.INSTALLED:
            raise

        utils.log('Error in Cryptoquip downloader', file=sys.stderr)
        traceback.print_exc()
        input('\n(Press ENTER to exit)\n> ')


if __name__ == "__main__":
    main()