from cryptoquip.update.check import UpdateContext
from cryptoquip.update.update import update
from types import SimpleNamespace

import argparse

from .. import config
from .. import utils


COLUMNS = config.config.menu.columns.resolve(2)

def choose_image(image_contexts, update_context):

    args = _parse_args()

    if args.skip_menu:
        return _skip_menu(image_contexts, args)

    menu_options = _to_menu_options(image_contexts)
    bi_menu      = _furcate(menu_options)

    usr_input      = _display_menu(bi_menu, update_context)
    selected_image = _select_option(usr_input, image_contexts, update_context)

    return selected_image

def _parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-s', '--skip-menu', action='store_true')
    parser.add_argument('-d', '--day', type=int, default=0)

    return parser.parse_args()

def _skip_menu(image_contexts, args):
    utils.log('Skipping menu')
    return image_contexts[args.day]

def _usr_in():
    return input(f'> ')


def _bail(msg):

    utils.log('Error:', msg)
    input('(Press Enter to exit)')
    exit(1)

def _to_menu_options(image_contexts):
    return [MenuOption(ic) for ic in image_contexts]

# yes, that is a word
def _furcate(menu_options):

    blanks = [BlankMenuOption(None)] * (len(menu_options) % COLUMNS)

    contexts = [*menu_options, *blanks]

    center = len(contexts) // COLUMNS

    return [
        tuple(contexts[ center*col + i] for col in range(COLUMNS))
        for i in range(center)
    ]

def _display_menu(bi_menu, update_context):

    utils.log('Which Crytoquip to download? (press Enter to download the most recent)')
    utils.log(_to_menu_str(bi_menu))
    
    if update_context.is_updateable:
        utils.log(_update_menu(update_context))

    return _usr_in()


def _to_menu_str(bi_menu):

    # for each column of the menu, determine the max size of the 3 sub columns
    def size_of(col, prop):
        return max(
            [
                len( getattr(row[col], prop) )
                for row in bi_menu
            ]
        )

    sub_col_sizes = [
        (
            size_of(col, 'ord_str'),
            size_of(col, 'day_str'),
            size_of(col, 'date_str'),
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

def _update_menu(update_context):
    return f'[There is an update available (ver {update_context.v_latest}).  Type "update" or "u" to install the update]'

def _select_option(usr_input, image_contexts, update_context):
    selection = _select_update(usr_input, update_context)

    print ('not selection', selection)

    if not selection:
        selection = _select_image(usr_input, image_contexts)
        
    return selection

def _select_update(usr_input, update_context):
    return update_context if usr_input.lower() in ['u', 'update'] else None

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