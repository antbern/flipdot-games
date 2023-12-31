//
// monospace_6pt
// Font Size: 5x9px
// Created: 24-05-2023 21:58:43
//

//
// Pseudocode for retrieving data for a specific character:
//
// offset = ascii_code(character) - ascii_code(' ')
// data = monospace_6pt[lut[offset]]
//

pub fn get_bytes_for_char(character: u8) -> &'static [u8] {
    let start = LUT[(character - 0x20) as usize] as usize;

    &MONOSPACE_6PT[start..start + 9]
}

pub const fn char_width() -> usize {
    5
}

const MONOSPACE_6PT: [u8; 774] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Character 0x20 (32: ' ')
    0x00, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04, 0x00, 0x00, // Character 0x21 (33: '!')
    0x00, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Character 0x27 (39: ''')
    0x00, 0x15, 0x0E, 0x0E, 0x15, 0x00, 0x00, 0x00, 0x00, // Character 0x2a (42: '*')
    0x00, 0x00, 0x04, 0x04, 0x1F, 0x04, 0x04, 0x00, 0x00, // Character 0x2b (43: '+')
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0x00, // Character 0x2c (44: ',')
    0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, // Character 0x2d (45: '-')
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, // Character 0x2e (46: '.')
    0x00, 0x08, 0x04, 0x04, 0x06, 0x02, 0x02, 0x01, 0x00, // Character 0x2f (47: '/')
    0x00, 0x0C, 0x12, 0x16, 0x12, 0x12, 0x0C, 0x00, 0x00, // Character 0x30 (48: '0')
    0x00, 0x06, 0x04, 0x04, 0x04, 0x04, 0x0E, 0x00, 0x00, // Character 0x31 (49: '1')
    0x00, 0x0E, 0x10, 0x10, 0x08, 0x04, 0x1E, 0x00, 0x00, // Character 0x32 (50: '2')
    0x00, 0x0E, 0x10, 0x10, 0x0C, 0x10, 0x1E, 0x00, 0x00, // Character 0x33 (51: '3')
    0x00, 0x08, 0x0C, 0x0C, 0x0A, 0x1E, 0x08, 0x00, 0x00, // Character 0x34 (52: '4')
    0x00, 0x1E, 0x02, 0x0E, 0x10, 0x10, 0x0E, 0x00, 0x00, // Character 0x35 (53: '5')
    0x00, 0x1C, 0x06, 0x02, 0x1E, 0x12, 0x1C, 0x00, 0x00, // Character 0x36 (54: '6')
    0x00, 0x1E, 0x10, 0x08, 0x08, 0x08, 0x04, 0x00, 0x00, // Character 0x37 (55: '7')
    0x00, 0x0C, 0x12, 0x12, 0x0C, 0x12, 0x1E, 0x00, 0x00, // Character 0x38 (56: '8')
    0x00, 0x0E, 0x12, 0x1E, 0x10, 0x18, 0x0E, 0x00, 0x00, // Character 0x39 (57: '9')
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x04, 0x00, 0x00, // Character 0x3a (58: ':')
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x04, 0x04, 0x00, // Character 0x3b (59: ';')
    0x00, 0x00, 0x00, 0x10, 0x0E, 0x06, 0x18, 0x00, 0x00, // Character 0x3c (60: '<')
    0x00, 0x00, 0x00, 0x0F, 0x00, 0x0F, 0x00, 0x00, 0x00, // Character 0x3d (61: '=')
    0x00, 0x00, 0x00, 0x02, 0x1C, 0x18, 0x06, 0x00, 0x00, // Character 0x3e (62: '>')
    0x00, 0x0E, 0x0C, 0x04, 0x04, 0x00, 0x04, 0x00, 0x00, // Character 0x3f (63: '?')
    0x00, 0x00, 0x0C, 0x12, 0x1A, 0x1A, 0x02, 0x0C, 0x00, // Character 0x40 (64: '@')
    0x00, 0x0C, 0x0C, 0x0C, 0x0C, 0x1E, 0x12, 0x00, 0x00, // Character 0x41 (65: 'A')
    0x00, 0x0E, 0x12, 0x12, 0x0E, 0x12, 0x1E, 0x00, 0x00, // Character 0x42 (66: 'B')
    0x00, 0x1C, 0x02, 0x02, 0x02, 0x02, 0x1C, 0x00, 0x00, // Character 0x43 (67: 'C')
    0x00, 0x0E, 0x12, 0x12, 0x12, 0x12, 0x0E, 0x00, 0x00, // Character 0x44 (68: 'D')
    0x00, 0x1E, 0x02, 0x02, 0x1E, 0x02, 0x1E, 0x00, 0x00, // Character 0x45 (69: 'E')
    0x00, 0x1E, 0x02, 0x02, 0x1E, 0x02, 0x02, 0x00, 0x00, // Character 0x46 (70: 'F')
    0x00, 0x1C, 0x02, 0x02, 0x1A, 0x12, 0x1C, 0x00, 0x00, // Character 0x47 (71: 'G')
    0x00, 0x12, 0x12, 0x12, 0x1E, 0x12, 0x12, 0x00, 0x00, // Character 0x48 (72: 'H')
    0x00, 0x0E, 0x04, 0x04, 0x04, 0x04, 0x0E, 0x00, 0x00, // Character 0x49 (73: 'I')
    0x00, 0x0C, 0x08, 0x08, 0x08, 0x08, 0x0E, 0x00, 0x00, // Character 0x4a (74: 'J')
    0x00, 0x12, 0x0A, 0x06, 0x0A, 0x0A, 0x12, 0x00, 0x00, // Character 0x4b (75: 'K')
    0x00, 0x02, 0x02, 0x02, 0x02, 0x02, 0x1E, 0x00, 0x00, // Character 0x4c (76: 'L')
    0x00, 0x12, 0x1E, 0x1E, 0x1E, 0x12, 0x12, 0x00, 0x00, // Character 0x4d (77: 'M')
    0x00, 0x12, 0x16, 0x16, 0x1A, 0x1A, 0x12, 0x00, 0x00, // Character 0x4e (78: 'N')
    0x00, 0x0C, 0x12, 0x12, 0x12, 0x12, 0x0C, 0x00, 0x00, // Character 0x4f (79: 'O')
    0x00, 0x1E, 0x12, 0x1E, 0x02, 0x02, 0x02, 0x00, 0x00, // Character 0x50 (80: 'P')
    0x00, 0x0C, 0x12, 0x12, 0x12, 0x12, 0x0C, 0x10, 0x00, // Character 0x51 (81: 'Q')
    0x00, 0x1E, 0x12, 0x0E, 0x1A, 0x12, 0x02, 0x00, 0x00, // Character 0x52 (82: 'R')
    0x00, 0x1C, 0x02, 0x0E, 0x18, 0x10, 0x1E, 0x00, 0x00, // Character 0x53 (83: 'S')
    0x00, 0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, // Character 0x54 (84: 'T')
    0x00, 0x12, 0x12, 0x12, 0x12, 0x12, 0x0C, 0x00, 0x00, // Character 0x55 (85: 'U')
    0x00, 0x12, 0x12, 0x0C, 0x0C, 0x0C, 0x0C, 0x00, 0x00, // Character 0x56 (86: 'V')
    0x00, 0x11, 0x11, 0x15, 0x0A, 0x0A, 0x0A, 0x00, 0x00, // Character 0x57 (87: 'W')
    0x00, 0x12, 0x0C, 0x0C, 0x0C, 0x0C, 0x12, 0x00, 0x00, // Character 0x58 (88: 'X')
    0x00, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04, 0x00, 0x00, // Character 0x59 (89: 'Y')
    0x00, 0x1E, 0x08, 0x08, 0x04, 0x04, 0x1E, 0x00, 0x00, // Character 0x5a (90: 'Z')
    0x00, 0x01, 0x02, 0x02, 0x06, 0x04, 0x04, 0x08, 0x00, // Character 0x5c (92: '\')
    0x00, 0x06, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Character 0x5e (94: '^')
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, // Character 0x5f (95: '_')
    0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Character 0x60 (96: '`')
    0x00, 0x00, 0x00, 0x1E, 0x1E, 0x12, 0x1E, 0x00, 0x00, // Character 0x61 (97: 'a')
    0x02, 0x02, 0x02, 0x0E, 0x12, 0x12, 0x0E, 0x00, 0x00, // Character 0x62 (98: 'b')
    0x00, 0x00, 0x00, 0x0C, 0x02, 0x02, 0x0C, 0x00, 0x00, // Character 0x63 (99: 'c')
    0x10, 0x10, 0x10, 0x1C, 0x12, 0x12, 0x1C, 0x00, 0x00, // Character 0x64 (100: 'd')
    0x00, 0x00, 0x00, 0x1C, 0x1E, 0x02, 0x1C, 0x00, 0x00, // Character 0x65 (101: 'e')
    0x18, 0x04, 0x04, 0x1E, 0x04, 0x04, 0x04, 0x00, 0x00, // Character 0x66 (102: 'f')
    0x00, 0x00, 0x00, 0x1C, 0x12, 0x12, 0x1C, 0x10, 0x0E, // Character 0x67 (103: 'g')
    0x02, 0x02, 0x02, 0x1E, 0x12, 0x12, 0x12, 0x00, 0x00, // Character 0x68 (104: 'h')
    0x04, 0x00, 0x00, 0x06, 0x04, 0x04, 0x0E, 0x00, 0x00, // Character 0x69 (105: 'i')
    0x04, 0x00, 0x00, 0x06, 0x04, 0x04, 0x04, 0x04, 0x07, // Character 0x6a (106: 'j')
    0x02, 0x02, 0x02, 0x1A, 0x0E, 0x0E, 0x1A, 0x00, 0x00, // Character 0x6b (107: 'k')
    0x06, 0x04, 0x04, 0x04, 0x04, 0x04, 0x1C, 0x00, 0x00, // Character 0x6c (108: 'l')
    0x00, 0x00, 0x00, 0x1E, 0x0A, 0x0A, 0x0A, 0x00, 0x00, // Character 0x6d (109: 'm')
    0x00, 0x00, 0x00, 0x1E, 0x12, 0x12, 0x12, 0x00, 0x00, // Character 0x6e (110: 'n')
    0x00, 0x00, 0x00, 0x0C, 0x12, 0x12, 0x0C, 0x00, 0x00, // Character 0x6f (111: 'o')
    0x00, 0x00, 0x00, 0x0E, 0x12, 0x12, 0x0E, 0x02, 0x02, // Character 0x70 (112: 'p')
    0x00, 0x00, 0x00, 0x1C, 0x12, 0x12, 0x1C, 0x10, 0x10, // Character 0x71 (113: 'q')
    0x00, 0x00, 0x00, 0x0E, 0x02, 0x02, 0x02, 0x00, 0x00, // Character 0x72 (114: 'r')
    0x00, 0x00, 0x00, 0x1E, 0x0E, 0x10, 0x1E, 0x00, 0x00, // Character 0x73 (115: 's')
    0x00, 0x00, 0x04, 0x1E, 0x04, 0x04, 0x1C, 0x00, 0x00, // Character 0x74 (116: 't')
    0x00, 0x00, 0x00, 0x12, 0x12, 0x12, 0x1E, 0x00, 0x00, // Character 0x75 (117: 'u')
    0x00, 0x00, 0x00, 0x12, 0x0C, 0x0C, 0x0C, 0x00, 0x00, // Character 0x76 (118: 'v')
    0x00, 0x00, 0x00, 0x11, 0x15, 0x0E, 0x0A, 0x00, 0x00, // Character 0x77 (119: 'w')
    0x00, 0x00, 0x00, 0x1E, 0x0C, 0x0C, 0x1E, 0x00, 0x00, // Character 0x78 (120: 'x')
    0x00, 0x00, 0x00, 0x12, 0x0C, 0x0C, 0x04, 0x04, 0x06, // Character 0x79 (121: 'y')
    0x00, 0x00, 0x00, 0x1E, 0x0C, 0x04, 0x1E, 0x00, 0x00, // Character 0x7a (122: 'z')
    0x0C, 0x04, 0x04, 0x02, 0x04, 0x04, 0x0C, 0x00, 0x00, // Character 0x7b (123: '{')
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x00, // Character 0x7c (124: '|')
    0x06, 0x04, 0x04, 0x08, 0x04, 0x04, 0x06, 0x00, 0x00, // Character 0x7d (125: '}')
    0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0x00, // Character 0x7e (126: '~')
];

const LUT: [u16; 95] = [
    0, // Character 0x20 (32: ' ')
    9, // Character 0x21 (33: '!')
    0, 0, 0, 0, 0, 18, // Character 0x27 (39: ''')
    0, 0, 27,  // Character 0x2a (42: '*')
    36,  // Character 0x2b (43: '+')
    45,  // Character 0x2c (44: ',')
    54,  // Character 0x2d (45: '-')
    63,  // Character 0x2e (46: '.')
    72,  // Character 0x2f (47: '/')
    81,  // Character 0x30 (48: '0')
    90,  // Character 0x31 (49: '1')
    99,  // Character 0x32 (50: '2')
    108, // Character 0x33 (51: '3')
    117, // Character 0x34 (52: '4')
    126, // Character 0x35 (53: '5')
    135, // Character 0x36 (54: '6')
    144, // Character 0x37 (55: '7')
    153, // Character 0x38 (56: '8')
    162, // Character 0x39 (57: '9')
    171, // Character 0x3a (58: ':')
    180, // Character 0x3b (59: ';')
    189, // Character 0x3c (60: '<')
    198, // Character 0x3d (61: '=')
    207, // Character 0x3e (62: '>')
    216, // Character 0x3f (63: '?')
    225, // Character 0x40 (64: '@')
    234, // Character 0x41 (65: 'A')
    243, // Character 0x42 (66: 'B')
    252, // Character 0x43 (67: 'C')
    261, // Character 0x44 (68: 'D')
    270, // Character 0x45 (69: 'E')
    279, // Character 0x46 (70: 'F')
    288, // Character 0x47 (71: 'G')
    297, // Character 0x48 (72: 'H')
    306, // Character 0x49 (73: 'I')
    315, // Character 0x4a (74: 'J')
    324, // Character 0x4b (75: 'K')
    333, // Character 0x4c (76: 'L')
    342, // Character 0x4d (77: 'M')
    351, // Character 0x4e (78: 'N')
    360, // Character 0x4f (79: 'O')
    369, // Character 0x50 (80: 'P')
    378, // Character 0x51 (81: 'Q')
    387, // Character 0x52 (82: 'R')
    396, // Character 0x53 (83: 'S')
    405, // Character 0x54 (84: 'T')
    414, // Character 0x55 (85: 'U')
    423, // Character 0x56 (86: 'V')
    432, // Character 0x57 (87: 'W')
    441, // Character 0x58 (88: 'X')
    450, // Character 0x59 (89: 'Y')
    459, // Character 0x5a (90: 'Z')
    0, 468, // Character 0x5c (92: '\')
    0, 477, // Character 0x5e (94: '^')
    486, // Character 0x5f (95: '_')
    495, // Character 0x60 (96: '`')
    504, // Character 0x61 (97: 'a')
    513, // Character 0x62 (98: 'b')
    522, // Character 0x63 (99: 'c')
    531, // Character 0x64 (100: 'd')
    540, // Character 0x65 (101: 'e')
    549, // Character 0x66 (102: 'f')
    558, // Character 0x67 (103: 'g')
    567, // Character 0x68 (104: 'h')
    576, // Character 0x69 (105: 'i')
    585, // Character 0x6a (106: 'j')
    594, // Character 0x6b (107: 'k')
    603, // Character 0x6c (108: 'l')
    612, // Character 0x6d (109: 'm')
    621, // Character 0x6e (110: 'n')
    630, // Character 0x6f (111: 'o')
    639, // Character 0x70 (112: 'p')
    648, // Character 0x71 (113: 'q')
    657, // Character 0x72 (114: 'r')
    666, // Character 0x73 (115: 's')
    675, // Character 0x74 (116: 't')
    684, // Character 0x75 (117: 'u')
    693, // Character 0x76 (118: 'v')
    702, // Character 0x77 (119: 'w')
    711, // Character 0x78 (120: 'x')
    720, // Character 0x79 (121: 'y')
    729, // Character 0x7a (122: 'z')
    738, // Character 0x7b (123: '{')
    747, // Character 0x7c (124: '|')
    756, // Character 0x7d (125: '}')
    765, // Character 0x7e (126: '~')
];
