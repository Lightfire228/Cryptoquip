from cryptoquip import app

import sys
import traceback

def main():
    try: 
        app.run()
    except KeyboardInterrupt:
        raise
    except:
        print('Error in Cryptoquip downloader', file=sys.stderr)
        traceback.print_exc()
        input('\n(Press ENTER to exit)\n> ')


if __name__ == "__main__":
    main()