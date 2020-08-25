from datetime import datetime, timedelta
from types    import SimpleNamespace

import argparse
import sys

def parse_args():

    if len(sys.argv) == 1:
        return interactive_menu()

    parser = argparse.ArgumentParser(description='')
    parser.add_argument('-d', '--days-ago', type=int, default=0)

    return parser.parse_args()

def interactive_menu():
    args = SimpleNamespace()

    options = 48

    def right_adj(i, n):
        s = f'{" " * n}{str(i)}'
        return s[-n:]

    def left_adj(i, n):
        s = f'{str(i)}{" " * n}'
        return s[:n]
        

    dates = [
        (i, datetime.today() - timedelta(days=i))
        for i in range(options)
    ]

    date_strings =[
        (i, f'{left_adj(d.strftime("%A"), 9)} - {d.strftime("%x")}')
        for i, d in dates 
    ]

    menu = [
        f'{right_adj(i, 2)} - {ds}'
        for i, ds in date_strings
    ]
    
    center = options // 2
    bimenu = ''.join([
        f'\n  {menu[i]} | {menu[i + center]}'
        for i in range(center)
    ])

    days_ago = usr_in('Which Crytoquip to download? (press Enter to download the most recent)' + bimenu)

    if len(days_ago) == 0:
        args.days_ago = 0
    else:
        msg = 'Must be a valid number between 0 and 47'

        if not days_ago.isdigit():
            bail(msg)
        
        days_ago = int(days_ago)

        if days_ago > 48 or days_ago < 0:
            bail(msg)
        
        args.days_ago = days_ago

    return args


def usr_in(msg):
    return input(f'{msg}\n> ')



def bail(msg):

    print('Error:', msg)
    input('(Press Enter to exit)')
    exit(1)

