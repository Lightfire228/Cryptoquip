from datetime import datetime, timedelta
from types    import SimpleNamespace

import argparse
import sys

COLUMNS = 2

def choose_image(image_contexts):

    args = _parse_args()

    if args.skip_menu:
        return _skip_menu(image_contexts, args)

    menu_options = _to_menu_options(image_contexts)
    bi_menu      = _bifurcate(menu_options)

    usr_input      = _display_menu(bi_menu)
    selected_image = _select_image(usr_input, image_contexts)

    return selected_image

def _parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--skip-menu', action='store_true')
    parser.add_argument('-d', '--day', type=int, default=0)

    return parser.parse_args()

def _skip_menu(image_contexts, args):
    print('Skipping menu')
    return image_contexts[args.day]

def _usr_in():
    return input(f'> ')


def _bail(msg):

    print('Error:', msg)
    input('(Press Enter to exit)')
    exit(1)

def _to_menu_options(image_contexts):
    return [MenuOption(ic) for ic in image_contexts]

def _bifurcate(menu_options):

    contexts = [*menu_options, BlankMenuOption(None)]

    center = len(contexts) // COLUMNS

    return [
        (contexts[i], contexts[center + i])
        for i in range(center)
    ]

def _display_menu(bi_menu):

    print('Which Crytoquip to download? (press Enter to download the most recent)')
    print(_to_menu_str(bi_menu))

    return _usr_in()


def _to_menu_str(bi_menu):

    # for each column of the menu, determine the max size of the 3 sub columns
    sub_col_sizes = [
        (
            max([len(row[col].ord_str)  for row in bi_menu]),
            max([len(row[col].day_str)  for row in bi_menu]),
            max([len(row[col].date_str) for row in bi_menu]),
        )
        for col in range(COLUMNS)
    ]

    # for each column of each row, format the column based on the max size *per* column
    col_strs = [
        [
            opt.format(sub_col_sizes[col])
            for col, opt in enumerate(row)
        ]
        for row in bi_menu
    ]

    # join each column together, then each row together
    menu_str = '  ' + '\n  '.join([
        ' | '.join(row)
        for row in col_strs
    ])

    return menu_str

def _select_image(usr_input, image_contexts):

    if len(usr_input) == 0:
        return image_contexts[0]

    max_ = len(image_contexts) -1

    try: 
        usr_input = int(usr_input)
        if 0 > usr_input < max_:
            raise Exception()
    
        return image_contexts[usr_input]
    except:
        _bail(f'Input must be a number between 0 and {max_}')


class MenuOption():
    
    def __init__(self, image_context):
        self.context = image_context
    
    @property
    def ordinal(self):
        return self.context.ordinal

    @property
    def ord_str(self):
        return str(self.ordinal)
    
    @property
    def day_str(self):
        return self.context.date.strftime('%A')
    
    @property
    def date_str(self):
        return self.context.date.strftime('%x')
    
    def format(self, col_sizes):

        (ord_col, day_col, date_col) = col_sizes

        ord  = self._align(self.ord_str,  ord_col,  'right')
        day  = self._align(self.day_str,  day_col,  'left')
        date = self._align(self.date_str, date_col, 'left')

        return f'{ord} - {day} - {date}'

    def _align(self, text, padding, side):

        # right adjust adds padding to the left of the word
        left_padding  = padding if side == 'right' else 0
        right_padding = padding if side == 'left'  else None

        left_space  = ' ' * left_padding
        right_space = ' ' * (right_padding or 0)

        s = left_space + text + right_space
        return s[-left_padding:right_padding]

class BlankMenuOption(MenuOption):

    def __init__(self, image_context):
        super().__init__(image_context)
    
    @property
    def ordinal(self):
        return ''
    
    @property
    def day_str(self):
        return ''
    
    @property
    def date_str(self):
        return ''

    def format(self, col_sizes):
        return ''
