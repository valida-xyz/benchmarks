use valida_machine::{ExtensionField, InstructionWord, Operands, PrimeField64};

pub fn generate_sha256_program<Val: PrimeField64, Challenge: ExtensionField<Val>>(
) -> Vec<InstructionWord<i32>> {
    let mut instructions = vec![];
    instructions.extend([
        InstructionWord {
            opcode: 7,
            operands: Operands([-36, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-40, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-44, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-48, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-52, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-56, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-60, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-64, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-68, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-72, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-76, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-80, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-84, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-88, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-92, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-96, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-100, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-104, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-108, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-112, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-116, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-120, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-124, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-128, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-132, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-136, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-140, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-144, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-148, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-152, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-156, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-160, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-164, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-168, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-172, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-176, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-180, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-184, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-188, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-192, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-196, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-200, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-204, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-208, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-212, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-216, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-220, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-224, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-228, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-232, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-236, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-240, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-244, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-248, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-252, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-256, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-260, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-264, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-268, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-272, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-276, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-280, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-284, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-288, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-4, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-8, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-12, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-16, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-20, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-24, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-28, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 7,
            operands: Operands([-32, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([0, 0, -4, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-8, 0, -12, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-16, 0, -20, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-24, 0, -28, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-36, -40, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-48, -52, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-60, -64, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-72, -76, -68, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-80, -84, -48, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-88, -92, -36, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-96, 0, -100, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-104, 0, -108, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-112, 0, -116, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-120, 0, -124, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-132, -136, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-144, -148, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-156, -160, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-168, -172, -164, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-176, -180, -144, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-184, -188, -132, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-192, 0, -196, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-200, 0, -204, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-208, 0, -212, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-216, 0, -220, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-228, -232, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-240, -244, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-252, -256, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-264, -268, -260, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-272, -276, -240, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-280, -284, -228, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-288, 0, -292, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-296, 0, -300, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-304, 0, -308, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-312, 0, -316, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-324, -328, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-336, -340, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-348, -352, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-360, -364, -356, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-368, -372, -336, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-376, -380, -324, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-384, 0, -388, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-392, 0, -396, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-400, 0, -404, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-408, 0, -412, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-420, -424, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-432, -436, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-444, -448, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-456, -460, -452, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-464, -468, -432, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-472, -476, -420, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-480, 0, -484, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-488, 0, -492, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-496, 0, -500, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-504, 0, -508, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-516, -520, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-528, -532, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-540, -544, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-552, -556, -548, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-560, -564, -528, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-568, -572, -516, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-576, 0, -580, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-584, 0, -588, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-592, 0, -596, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-600, 0, -604, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-612, -616, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-624, -628, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-636, -640, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-648, -652, -644, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-656, -660, -624, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-664, -668, -612, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-672, 0, -676, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-680, 0, -684, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-688, 0, -692, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-696, 0, -700, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-708, -712, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-720, -724, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-732, -736, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-744, -748, -740, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-752, -756, -720, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-760, -764, -708, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-768, 0, -772, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-776, 0, -780, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-784, 0, -788, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-792, 0, -796, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-804, -808, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-816, -820, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-828, -832, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-840, -844, -836, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-848, -852, -816, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-856, -860, -804, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-864, 0, -868, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-872, 0, -876, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-880, 0, -884, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-888, 0, -892, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-900, -904, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-912, -916, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-924, -928, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-936, -940, -932, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-944, -948, -912, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-952, -956, -900, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-960, 0, -964, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-968, 0, -972, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-976, 0, -980, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-984, 0, -988, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-996, -1000, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1008, -1012, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1020, -1024, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1032, -1036, -1028, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1040, -1044, -1008, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1048, -1052, -996, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1056, 0, -1060, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1064, 0, -1068, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1072, 0, -1076, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1080, 0, -1084, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1092, -1096, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1104, -1108, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1116, -1120, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1128, -1132, -1124, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1136, -1140, -1104, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1144, -1148, -1092, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1152, 0, -1156, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1160, 0, -1164, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1168, 0, -1172, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1176, 0, -1180, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1188, -1192, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1200, -1204, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1212, -1216, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1224, -1228, -1220, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1232, -1236, -1200, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1240, -1244, -1188, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1248, 0, -1252, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1256, 0, -1260, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1264, 0, -1268, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1272, 0, -1276, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1284, -1288, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1296, -1300, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1308, -1312, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1320, -1324, -1316, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1328, -1332, -1296, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1336, -1340, -1284, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1344, 0, -1348, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1352, 0, -1356, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1360, 0, -1364, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1368, 0, -1372, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1380, -1384, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1392, -1396, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1404, -1408, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1416, -1420, -1412, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1424, -1428, -1392, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1432, -1436, -1380, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1440, 0, -1444, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1448, 0, -1452, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1456, 0, -1460, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-1464, 0, -1468, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1476, -1480, 24, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1488, -1492, 16, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1500, -1504, 8, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1512, -1516, -1508, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1520, -1524, -1488, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1528, -1532, -1476, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1536, -1540, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1544, -1540, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1548, -1536, -1544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1536, -1540, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1544, -1540, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1552, -1536, -1544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1556, -1560, -1552, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1564, -1540, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1568, -1572, -1564, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1576, -1580, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1584, -1580, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1588, -1576, -1584, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1576, -1580, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1584, -1580, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1592, -1576, -1584, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1596, -1600, -1592, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1604, -1580, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1608, -1612, -1604, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1616, -1620, -1624, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1628, -1632, -1636, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1640, -1644, -1568, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1648, -1652, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1656, -1652, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1660, -1648, -1656, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1648, -1652, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1656, -1652, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1664, -1648, -1656, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1668, -1672, -1664, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1676, -1652, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1680, -1684, -1676, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1688, -1692, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1696, -1692, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1700, -1688, -1696, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1688, -1692, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1696, -1692, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1704, -1688, -1696, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1708, -1712, -1704, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1716, -1692, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1720, -1724, -1716, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1728, -1732, -1736, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1740, -1744, -1748, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1752, -1756, -1680, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1760, -1764, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1768, -1764, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1772, -1760, -1768, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1760, -1764, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1768, -1764, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1776, -1760, -1768, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1780, -1784, -1776, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1788, -1764, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1792, -1796, -1788, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1800, -1804, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1808, -1804, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1812, -1800, -1808, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1800, -1804, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1808, -1804, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1816, -1800, -1808, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1820, -1824, -1816, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1828, -1804, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1832, -1836, -1828, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1840, -1844, -1848, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1852, -1856, -1860, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1864, -1868, -1792, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1872, -1876, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1880, -1876, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1884, -1872, -1880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1872, -1876, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1880, -1876, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1888, -1872, -1880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1892, -1896, -1888, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1900, -1876, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1904, -1908, -1900, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1912, -1916, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1920, -1916, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1924, -1912, -1920, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1912, -1916, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1920, -1916, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1928, -1912, -1920, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1932, -1936, -1928, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1940, -1916, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-1944, -1948, -1940, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1952, -1956, -1960, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1964, -1968, -1972, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-1976, -1980, -1904, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1984, -1988, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1992, -1988, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-1996, -1984, -1992, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1984, -1988, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1992, -1988, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2000, -1984, -1992, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2004, -2008, -2000, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2012, -1988, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2016, -2020, -2012, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2024, -2028, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2032, -2028, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2036, -2024, -2032, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2024, -2028, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2032, -2028, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2040, -2024, -2032, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2044, -2048, -2040, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2052, -2028, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2056, -2060, -2052, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2064, -2068, -2072, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2076, -2080, -2084, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2088, -2092, -2016, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2096, -2100, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2104, -2100, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2108, -2096, -2104, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2096, -2100, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2104, -2100, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2112, -2096, -2104, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2116, -2120, -2112, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2124, -2100, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2128, -2132, -2124, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2136, -2140, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2144, -2140, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2148, -2136, -2144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2136, -2140, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2144, -2140, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2152, -2136, -2144, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2156, -2160, -2152, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2164, -2140, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2168, -2172, -2164, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2176, -2180, -2184, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2188, -2192, -2196, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2200, -2204, -2128, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2208, -2212, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2216, -2212, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2220, -2208, -2216, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2208, -2212, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2216, -2212, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2224, -2208, -2216, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2228, -2232, -2224, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2236, -2212, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2240, -2244, -2236, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2248, -2252, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2256, -2252, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2260, -2248, -2256, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2248, -2252, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2256, -2252, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2264, -2248, -2256, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2268, -2272, -2264, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2276, -2252, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2280, -2284, -2276, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2288, -2292, -2296, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2300, -2304, -2308, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2312, -2316, -2240, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2320, -2324, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2328, -2324, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2332, -2320, -2328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2320, -2324, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2328, -2324, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2336, -2320, -2328, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2340, -2344, -2336, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2348, -2324, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2352, -2356, -2348, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2360, -2364, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2368, -2364, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2372, -2360, -2368, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2360, -2364, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2368, -2364, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2376, -2360, -2368, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2380, -2384, -2376, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2388, -2364, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2392, -2396, -2388, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2400, -2404, -2408, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2412, -2416, -1640, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2420, -2424, -2352, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2428, -2432, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2436, -2432, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2440, -2428, -2436, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2428, -2432, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2436, -2432, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2444, -2428, -2436, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2448, -2452, -2444, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2456, -2432, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2460, -2464, -2456, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2468, -2472, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2476, -2472, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2480, -2468, -2476, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2468, -2472, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2476, -2472, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2484, -2468, -2476, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2488, -2492, -2484, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2496, -2472, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2500, -2504, -2496, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2508, -2512, -2516, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2520, -2524, -1752, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2528, -2532, -2460, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2536, -2540, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2544, -2540, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2548, -2536, -2544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2536, -2540, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2544, -2540, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2552, -2536, -2544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2556, -2560, -2552, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2564, -2540, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2568, -2572, -2564, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2576, -2580, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2584, -2580, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2588, -2576, -2584, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2576, -2580, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2584, -2580, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2592, -2576, -2584, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2596, -2600, -2592, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2604, -2580, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2608, -2612, -2604, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2616, -2620, -1636, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2624, -2628, -1864, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2632, -2636, -2568, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2640, -2644, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2648, -2644, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2652, -2640, -2648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2640, -2644, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2648, -2644, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2656, -2640, -2648, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2660, -2664, -2656, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2668, -2644, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2672, -2676, -2668, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2680, -2684, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2688, -2684, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2692, -2680, -2688, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2680, -2684, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2688, -2684, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2696, -2680, -2688, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2700, -2704, -2696, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2708, -2684, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2712, -2716, -2708, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2720, -2724, -1748, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2728, -2732, -1976, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2736, -2740, -2672, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2744, -2748, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2752, -2748, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2756, -2744, -2752, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2744, -2748, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2752, -2748, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2760, -2744, -2752, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2764, -2768, -2760, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2772, -2748, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2776, -2780, -2772, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2784, -2788, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2792, -2788, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2796, -2784, -2792, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2784, -2788, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2792, -2788, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2800, -2784, -2792, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2804, -2808, -2800, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2812, -2788, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2816, -2820, -2812, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2824, -2828, -1860, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2832, -2836, -2088, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2840, -2844, -2776, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2848, -2852, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2856, -2852, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2860, -2848, -2856, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2848, -2852, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2856, -2852, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2864, -2848, -2856, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2868, -2872, -2864, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2876, -2852, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2880, -2884, -2876, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2888, -2892, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2896, -2892, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2900, -2888, -2896, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2888, -2892, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2896, -2892, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2904, -2888, -2896, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2908, -2912, -2904, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2916, -2892, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2920, -2924, -2916, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2928, -2932, -1972, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2936, -2940, -2200, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-2944, -2948, -2880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2952, -2956, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2960, -2956, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2964, -2952, -2960, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2952, -2956, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2960, -2956, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2968, -2952, -2960, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2972, -2976, -2968, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2980, -2956, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-2984, -2988, -2980, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1536, -1540, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1544, -1540, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2992, -1536, -1544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1536, -1540, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1544, -1540, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-2996, -1536, -1544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3000, -3004, -2996, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3008, -1540, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3012, -3016, -3008, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3020, -3024, -2084, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3028, -3032, -2312, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3036, -3040, -2984, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3044, -3048, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3052, -3048, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3056, -3044, -3052, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3044, -3048, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3052, -3048, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3060, -3044, -3052, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3064, -3068, -3060, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3072, -3048, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3076, -3080, -3072, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1648, -1652, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1656, -1652, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3084, -1648, -1656, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1648, -1652, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1656, -1652, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3088, -1648, -1656, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3092, -3096, -3088, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3100, -1652, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3104, -3108, -3100, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3112, -3116, -2196, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3120, -3124, -2420, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3128, -3132, -3076, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3136, -3140, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3144, -3140, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3148, -3136, -3144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3136, -3140, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3144, -3140, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3152, -3136, -3144, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3156, -3160, -3152, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3164, -3140, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3168, -3172, -3164, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1760, -1764, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1768, -1764, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3176, -1760, -1768, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1760, -1764, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1768, -1764, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3180, -1760, -1768, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3184, -3188, -3180, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3192, -1764, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3196, -3200, -3192, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3204, -3208, -2308, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3212, -3216, -2528, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3220, -3224, -3168, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3228, -3232, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3236, -3232, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3240, -3228, -3236, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3228, -3232, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3236, -3232, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3244, -3228, -3236, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3248, -3252, -3244, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3256, -3232, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3260, -3264, -3256, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1872, -1876, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1880, -1876, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3268, -1872, -1880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1872, -1876, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1880, -1876, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3272, -1872, -1880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3276, -3280, -3272, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3284, -1876, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3288, -3292, -3284, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3296, -3300, -1640, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3304, -3308, -2632, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3312, -3316, -3260, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3320, -3324, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3328, -3324, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3332, -3320, -3328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3320, -3324, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3328, -3324, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3336, -3320, -3328, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3340, -3344, -3336, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3348, -3324, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3352, -3356, -3348, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1984, -1988, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1992, -1988, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3360, -1984, -1992, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-1984, -1988, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-1992, -1988, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3364, -1984, -1992, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3368, -3372, -3364, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3376, -1988, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3380, -3384, -3376, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3388, -3392, -1752, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3396, -3400, -2736, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3404, -3408, -3352, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3412, -3416, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3420, -3416, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3424, -3412, -3420, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3412, -3416, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3420, -3416, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3428, -3412, -3420, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3432, -3436, -3428, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3440, -3416, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3444, -3448, -3440, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2096, -2100, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2104, -2100, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3452, -2096, -2104, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2096, -2100, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2104, -2100, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3456, -2096, -2104, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3460, -3464, -3456, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3468, -2100, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3472, -3476, -3468, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3480, -3484, -1864, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3488, -3492, -2840, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3496, -3500, -3444, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3504, -3508, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3512, -3508, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3516, -3504, -3512, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3504, -3508, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3512, -3508, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3520, -3504, -3512, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3524, -3528, -3520, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3532, -3508, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3536, -3540, -3532, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2208, -2212, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2216, -2212, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3544, -2208, -2216, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2208, -2212, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2216, -2212, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3548, -2208, -2216, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3552, -3556, -3548, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3560, -2212, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3564, -3568, -3560, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3572, -3576, -1976, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3580, -3584, -2944, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3588, -3592, -3536, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3596, -3600, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3604, -3600, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3608, -3596, -3604, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3596, -3600, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3604, -3600, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3612, -3596, -3604, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3616, -3620, -3612, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3624, -3600, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3628, -3632, -3624, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2320, -2324, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2328, -2324, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3636, -2320, -2328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2320, -2324, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2328, -2324, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3640, -2320, -2328, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3644, -3648, -3640, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3652, -2324, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3656, -3660, -3652, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3664, -3668, -2088, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3672, -3676, -3036, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3680, -3684, -3628, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3688, -3692, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3696, -3692, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3700, -3688, -3696, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3688, -3692, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3696, -3692, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3704, -3688, -3696, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3708, -3712, -3704, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3716, -3692, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3720, -3724, -3716, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2428, -2432, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2436, -2432, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3728, -2428, -2436, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2428, -2432, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2436, -2432, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3732, -2428, -2436, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3736, -3740, -3732, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3744, -2432, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3748, -3752, -3744, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3756, -3760, -2200, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3764, -3768, -3128, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3772, -3776, -3720, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3780, -3784, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3788, -3784, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3792, -3780, -3788, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3780, -3784, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3788, -3784, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3796, -3780, -3788, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3800, -3804, -3796, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3808, -3784, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3812, -3816, -3808, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2536, -2540, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2544, -2540, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3820, -2536, -2544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2536, -2540, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2544, -2540, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3824, -2536, -2544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3828, -3832, -3824, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3836, -2540, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3840, -3844, -3836, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3848, -3852, -2312, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3856, -3860, -3220, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3864, -3868, -3812, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3872, -3876, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3880, -3876, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3884, -3872, -3880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3872, -3876, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3880, -3876, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3888, -3872, -3880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3892, -3896, -3888, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3900, -3876, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3904, -3908, -3900, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2640, -2644, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2648, -2644, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3912, -2640, -2648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2640, -2644, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2648, -2644, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3916, -2640, -2648, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3920, -3924, -3916, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3928, -2644, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3932, -3936, -3928, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3940, -3944, -2420, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3948, -3952, -3312, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-3956, -3960, -3904, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3964, -3968, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3972, -3968, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3976, -3964, -3972, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3964, -3968, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3972, -3968, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-3980, -3964, -3972, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3984, -3988, -3980, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3992, -3968, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-3996, -4000, -3992, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2744, -2748, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2752, -2748, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4004, -2744, -2752, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2744, -2748, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2752, -2748, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4008, -2744, -2752, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4012, -4016, -4008, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4020, -2748, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4024, -4028, -4020, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4032, -4036, -2528, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4040, -4044, -3404, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4048, -4052, -3996, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4056, -4060, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4064, -4060, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4068, -4056, -4064, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4056, -4060, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4064, -4060, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4072, -4056, -4064, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4076, -4080, -4072, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4084, -4060, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4088, -4092, -4084, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2848, -2852, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2856, -2852, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4096, -2848, -2856, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2848, -2852, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2856, -2852, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4100, -2848, -2856, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4104, -4108, -4100, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4112, -2852, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4116, -4120, -4112, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4124, -4128, -2632, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4132, -4136, -3496, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4140, -4144, -4088, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4148, -4152, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4156, -4152, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4160, -4148, -4156, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4148, -4152, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4156, -4152, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4164, -4148, -4156, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4168, -4172, -4164, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4176, -4152, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4180, -4184, -4176, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2952, -2956, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2960, -2956, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4188, -2952, -2960, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-2952, -2956, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-2960, -2956, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4192, -2952, -2960, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4196, -4200, -4192, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4204, -2956, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4208, -4212, -4204, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4216, -4220, -2736, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4224, -4228, -3588, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4232, -4236, -4180, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4240, -4244, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4248, -4244, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4252, -4240, -4248, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4240, -4244, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4248, -4244, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4256, -4240, -4248, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4260, -4264, -4256, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4268, -4244, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4272, -4276, -4268, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3044, -3048, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3052, -3048, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4280, -3044, -3052, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3044, -3048, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3052, -3048, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4284, -3044, -3052, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4288, -4292, -4284, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4296, -3048, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4300, -4304, -4296, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4308, -4312, -2840, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4316, -4320, -3680, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4324, -4328, -4272, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4332, -4336, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4340, -4336, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4344, -4332, -4340, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4332, -4336, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4340, -4336, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4348, -4332, -4340, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4352, -4356, -4348, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4360, -4336, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4364, -4368, -4360, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3136, -3140, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3144, -3140, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4372, -3136, -3144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3136, -3140, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3144, -3140, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4376, -3136, -3144, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4380, -4384, -4376, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4388, -3140, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4392, -4396, -4388, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4400, -4404, -2944, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4408, -4412, -3772, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4416, -4420, -4364, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4424, -4428, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4432, -4428, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4436, -4424, -4432, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4424, -4428, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4432, -4428, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4440, -4424, -4432, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4444, -4448, -4440, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4452, -4428, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4456, -4460, -4452, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3228, -3232, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3236, -3232, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4464, -3228, -3236, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3228, -3232, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3236, -3232, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4468, -3228, -3236, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4472, -4476, -4468, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4480, -3232, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4484, -4488, -4480, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4492, -4496, -3036, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4500, -4504, -3864, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4508, -4512, -4456, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4516, -4520, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4524, -4520, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4528, -4516, -4524, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4516, -4520, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4524, -4520, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4532, -4516, -4524, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4536, -4540, -4532, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4544, -4520, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4548, -4552, -4544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3320, -3324, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3328, -3324, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4556, -3320, -3328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3320, -3324, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3328, -3324, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4560, -3320, -3328, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4564, -4568, -4560, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4572, -3324, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4576, -4580, -4572, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4584, -4588, -3128, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4592, -4596, -3956, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4600, -4604, -4548, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4608, -4612, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4616, -4612, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4620, -4608, -4616, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4608, -4612, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4616, -4612, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4624, -4608, -4616, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4628, -4632, -4624, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4636, -4612, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4640, -4644, -4636, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3412, -3416, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3420, -3416, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4648, -3412, -3420, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3412, -3416, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3420, -3416, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4652, -3412, -3420, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4656, -4660, -4652, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4664, -3416, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4668, -4672, -4664, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4676, -4680, -3220, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4684, -4688, -4048, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4692, -4696, -4640, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4700, -4704, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4708, -4704, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4712, -4700, -4708, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4700, -4704, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4708, -4704, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4716, -4700, -4708, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4720, -4724, -4716, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4728, -4704, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4732, -4736, -4728, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3504, -3508, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3512, -3508, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4740, -3504, -3512, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3504, -3508, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3512, -3508, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4744, -3504, -3512, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4748, -4752, -4744, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4756, -3508, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4760, -4764, -4756, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4768, -4772, -3312, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4776, -4780, -4140, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4784, -4788, -4732, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4792, -4796, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4800, -4796, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4804, -4792, -4800, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4792, -4796, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4800, -4796, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4808, -4792, -4800, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4812, -4816, -4808, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4820, -4796, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4824, -4828, -4820, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3596, -3600, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3604, -3600, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4832, -3596, -3604, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3596, -3600, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3604, -3600, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4836, -3596, -3604, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4840, -4844, -4836, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4848, -3600, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4852, -4856, -4848, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4860, -4864, -3404, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4868, -4872, -4232, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4876, -4880, -4824, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4884, -4888, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4892, -4888, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4896, -4884, -4892, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4884, -4888, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4892, -4888, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4900, -4884, -4892, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4904, -4908, -4900, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4912, -4888, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4916, -4920, -4912, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3688, -3692, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3696, -3692, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4924, -3688, -3696, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3688, -3692, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3696, -3692, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4928, -3688, -3696, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4932, -4936, -4928, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4940, -3692, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4944, -4948, -4940, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4952, -4956, -3496, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4960, -4964, -4324, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-4968, -4972, -4916, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4976, -4980, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4984, -4980, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4988, -4976, -4984, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4976, -4980, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4984, -4980, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-4992, -4976, -4984, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-4996, -5000, -4992, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5004, -4980, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5008, -5012, -5004, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3780, -3784, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3788, -3784, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5016, -3780, -3788, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3780, -3784, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3788, -3784, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5020, -3780, -3788, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5024, -5028, -5020, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5032, -3784, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5036, -5040, -5032, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5044, -5048, -3588, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5052, -5056, -4416, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5060, -5064, -5008, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5068, -5072, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5076, -5072, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5080, -5068, -5076, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5068, -5072, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5076, -5072, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5084, -5068, -5076, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5088, -5092, -5084, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5096, -5072, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5100, -5104, -5096, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3872, -3876, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3880, -3876, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5108, -3872, -3880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3872, -3876, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3880, -3876, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5112, -3872, -3880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5116, -5120, -5112, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5124, -3876, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5128, -5132, -5124, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5136, -5140, -3680, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5144, -5148, -4508, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5152, -5156, -5100, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5160, -5164, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5168, -5164, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5172, -5160, -5168, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5160, -5164, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5168, -5164, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5176, -5160, -5168, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5180, -5184, -5176, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5188, -5164, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5192, -5196, -5188, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3964, -3968, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3972, -3968, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5200, -3964, -3972, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-3964, -3968, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-3972, -3968, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5204, -3964, -3972, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5208, -5212, -5204, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5216, -3968, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5220, -5224, -5216, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5228, -5232, -3772, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5236, -5240, -4600, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5244, -5248, -5192, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5252, -5256, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5260, -5256, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5264, -5252, -5260, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5252, -5256, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5260, -5256, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5268, -5252, -5260, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5272, -5276, -5268, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5280, -5256, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5284, -5288, -5280, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4056, -4060, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4064, -4060, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5292, -4056, -4064, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4056, -4060, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4064, -4060, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5296, -4056, -4064, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5300, -5304, -5296, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5308, -4060, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5312, -5316, -5308, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5320, -5324, -3864, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5328, -5332, -4692, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5336, -5340, -5284, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5344, -5348, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5352, -5348, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5356, -5344, -5352, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5344, -5348, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5352, -5348, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5360, -5344, -5352, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5364, -5368, -5360, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5372, -5348, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5376, -5380, -5372, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4148, -4152, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4156, -4152, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5384, -4148, -4156, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4148, -4152, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4156, -4152, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5388, -4148, -4156, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5392, -5396, -5388, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5400, -4152, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5404, -5408, -5400, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5412, -5416, -3956, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5420, -5424, -4784, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5428, -5432, -5376, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5436, -5440, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5444, -5440, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5448, -5436, -5444, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5436, -5440, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5444, -5440, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5452, -5436, -5444, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5456, -5460, -5452, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5464, -5440, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5468, -5472, -5464, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4240, -4244, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4248, -4244, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5476, -4240, -4248, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4240, -4244, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4248, -4244, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5480, -4240, -4248, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5484, -5488, -5480, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5492, -4244, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5496, -5500, -5492, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5504, -5508, -4048, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5512, -5516, -4876, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5520, -5524, -5468, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5528, -5532, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5536, -5532, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5540, -5528, -5536, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5528, -5532, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5536, -5532, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5544, -5528, -5536, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5548, -5552, -5544, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5556, -5532, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5560, -5564, -5556, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4332, -4336, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4340, -4336, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5568, -4332, -4340, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4332, -4336, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4340, -4336, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5572, -4332, -4340, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5576, -5580, -5572, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5584, -4336, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5588, -5592, -5584, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5596, -5600, -4140, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5604, -5608, -4968, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5612, -5616, -5560, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5620, -5624, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5628, -5624, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5632, -5620, -5628, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5620, -5624, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5628, -5624, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5636, -5620, -5628, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5640, -5644, -5636, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5648, -5624, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5652, -5656, -5648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4424, -4428, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4432, -4428, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5660, -4424, -4432, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4424, -4428, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4432, -4428, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5664, -4424, -4432, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5668, -5672, -5664, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5676, -4428, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5680, -5684, -5676, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5688, -5692, -4232, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5696, -5700, -5060, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5704, -5708, -5652, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5712, -5716, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5720, -5716, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5724, -5712, -5720, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5712, -5716, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5720, -5716, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5728, -5712, -5720, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5732, -5736, -5728, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5740, -5716, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5744, -5748, -5740, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4516, -4520, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4524, -4520, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5752, -4516, -4524, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4516, -4520, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4524, -4520, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5756, -4516, -4524, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5760, -5764, -5756, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5768, -4520, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5772, -5776, -5768, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5780, -5784, -4324, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5788, -5792, -5152, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5796, -5800, -5744, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5804, -5808, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5812, -5808, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5816, -5804, -5812, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5804, -5808, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5812, -5808, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5820, -5804, -5812, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5824, -5828, -5820, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5832, -5808, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5836, -5840, -5832, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4608, -4612, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4616, -4612, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5844, -4608, -4616, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4608, -4612, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4616, -4612, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5848, -4608, -4616, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5852, -5856, -5848, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5860, -4612, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5864, -5868, -5860, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5872, -5876, -4416, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5880, -5884, -5244, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5888, -5892, -5836, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5896, -5900, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5904, -5900, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5908, -5896, -5904, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5896, -5900, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5904, -5900, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5912, -5896, -5904, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5916, -5920, -5912, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5924, -5900, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5928, -5932, -5924, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4700, -4704, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4708, -4704, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5936, -4700, -4708, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4700, -4704, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4708, -4704, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-5940, -4700, -4708, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5944, -5948, -5940, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5952, -4704, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-5956, -5960, -5952, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5964, -5968, -4508, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5972, -5976, -5336, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-5980, -5984, -5928, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5988, -5992, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5996, -5992, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6000, -5988, -5996, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-5988, -5992, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-5996, -5992, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6004, -5988, -5996, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6008, -6012, -6004, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6016, -5992, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6020, -6024, -6016, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4792, -4796, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4800, -4796, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6028, -4792, -4800, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4792, -4796, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4800, -4796, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6032, -4792, -4800, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6036, -6040, -6032, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6044, -4796, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6048, -6052, -6044, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6056, -6060, 15, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6064, -6060, 17, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6068, -6056, -6064, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6056, -6060, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6064, -6060, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6072, -6056, -6064, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6076, -6080, -6072, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6084, -6060, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6088, -6092, -6084, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4884, -4888, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4892, -4888, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6096, -4884, -4892, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-4884, -4888, 14, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-4892, -4888, 18, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6100, -4884, -4892, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6104, -6108, -6100, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6112, -4888, 3, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6116, -6120, -6112, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6124, 0, -6128, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6132, 0, -6136, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6140, 0, -6144, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6148, 0, -6152, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6156, 0, -6160, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6164, 0, -6168, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6172, 0, -6176, 0, 0]),
        },
        InstructionWord {
            opcode: 1,
            operands: Operands([-6180, 0, -6184, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6188, -6192, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6196, -6192, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6200, -6188, -6196, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6188, -6192, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6196, -6192, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6204, -6188, -6196, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6208, -6212, -6204, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6188, -6192, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6196, -6192, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6216, -6188, -6196, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6220, -6224, -6216, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6228, -6192, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6232, -6236, 1116352408, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6240, -6244, -6248, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6252, -6256, -6220, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6260, -6264, -6180, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6268, -6272, -6276, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6280, -6284, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6288, -6284, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6292, -6280, -6288, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6280, -6284, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6288, -6284, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6296, -6280, -6288, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6300, -6304, -6296, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6280, -6284, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6288, -6284, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6308, -6280, -6288, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6312, -6316, -6308, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6320, -6324, -6132, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6328, -6332, -6336, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6340, -6344, -6312, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6348, -6352, -6148, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6356, -6360, -6268, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6364, -6368, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6372, -6368, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6376, -6364, -6372, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6364, -6368, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6372, -6368, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6380, -6364, -6372, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6384, -6388, -6380, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6364, -6368, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6372, -6368, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6392, -6364, -6372, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6396, -6400, -6392, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6404, -6368, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6408, -1580, 1899447441, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6412, -6416, -6172, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6420, -6424, -6428, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6432, -6436, -6440, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6444, -6448, -6396, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6452, -6456, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6460, -6456, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6464, -6452, -6460, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6452, -6456, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6460, -6456, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6468, -6452, -6460, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6472, -6476, -6468, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6452, -6456, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6460, -6456, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6480, -6452, -6460, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6484, -6488, -6480, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6492, -6496, -6124, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6500, -6504, -6508, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6512, -6516, -6500, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6520, -6524, -6140, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6528, -6532, -6444, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6536, -6540, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6544, -6540, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6548, -6536, -6544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6536, -6540, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6544, -6540, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6552, -6536, -6544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6556, -6560, -6552, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6536, -6540, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6544, -6540, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6564, -6536, -6544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6568, -6572, -6564, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6576, -6540, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6580, -1692, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6584, -6588, -6164, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6592, -6596, -6600, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6604, -6608, -6612, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6616, -6620, -6568, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6624, -6628, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6632, -6628, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6636, -6624, -6632, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6624, -6628, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6632, -6628, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6640, -6624, -6632, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6644, -6648, -6640, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6624, -6628, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6632, -6628, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6652, -6624, -6632, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6656, -6660, -6652, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6664, -6456, -6124, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6668, -6672, -6676, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6680, -6684, -6668, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6688, -6692, -6132, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6696, -6700, -6616, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6704, -6708, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6712, -6708, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6716, -6704, -6712, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6704, -6708, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6712, -6708, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6720, -6704, -6712, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6724, -6728, -6720, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6704, -6708, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6712, -6708, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6732, -6704, -6712, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6736, -6740, -6732, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6744, -6708, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6748, -1804, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6752, -6756, -6156, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6760, -6764, -6768, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6772, -6776, -6780, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6784, -6788, -6736, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6792, -6796, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6800, -6796, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6804, -6792, -6800, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6792, -6796, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6800, -6796, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6808, -6792, -6800, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6812, -6816, -6808, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6792, -6796, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6800, -6796, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6820, -6792, -6800, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6824, -6828, -6820, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6832, -6628, -6356, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6836, -6840, -6844, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6848, -6852, -6836, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6856, -6860, -6124, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6864, -6868, -6784, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6872, -6876, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6880, -6876, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6884, -6872, -6880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6872, -6876, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6880, -6876, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6888, -6872, -6880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6892, -6896, -6888, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6872, -6876, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6880, -6876, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6900, -6872, -6880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6904, -6908, -6900, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6912, -6876, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6916, -1916, 961987163, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6920, -6924, -6348, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6928, -6932, -6936, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6940, -6944, -6948, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-6952, -6956, -6904, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6960, -6964, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6968, -6964, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6972, -6960, -6968, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6960, -6964, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6968, -6964, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6976, -6960, -6968, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6980, -6984, -6976, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-6960, -6964, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-6968, -6964, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-6988, -6960, -6968, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-6992, -6996, -6988, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7000, -6796, -6528, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7004, -7008, -7012, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7016, -7020, -7004, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7024, -7028, -6356, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7032, -7036, -6952, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7040, -7044, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7048, -7044, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7052, -7040, -7048, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7040, -7044, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7048, -7044, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7056, -7040, -7048, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7060, -7064, -7056, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7040, -7044, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7048, -7044, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7068, -7040, -7048, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7072, -7076, -7068, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7080, -7044, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7084, -2028, 1508970993, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7088, -7092, -6520, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7096, -7100, -7104, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7108, -7112, -7116, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7120, -7124, -7072, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7128, -7132, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7136, -7132, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7140, -7128, -7136, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7128, -7132, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7136, -7132, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7144, -7128, -7136, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7148, -7152, -7144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7128, -7132, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7136, -7132, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7156, -7128, -7136, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7160, -7164, -7156, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7168, -6964, -6696, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7172, -7176, -7180, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7184, -7188, -7172, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7192, -7196, -6528, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7200, -7204, -7120, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7208, -7212, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7216, -7212, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7220, -7208, -7216, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7208, -7212, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7216, -7212, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7224, -7208, -7216, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7228, -7232, -7224, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7208, -7212, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7216, -7212, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7236, -7208, -7216, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7240, -7244, -7236, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7248, -7212, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7252, -2140, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7256, -7260, -6688, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7264, -7268, -7272, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7276, -7280, -7284, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7288, -7292, -7240, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7296, -7300, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7304, -7300, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7308, -7296, -7304, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7296, -7300, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7304, -7300, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7312, -7296, -7304, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7316, -7320, -7312, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7296, -7300, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7304, -7300, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7324, -7296, -7304, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7328, -7332, -7324, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7336, -7132, -6864, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7340, -7344, -7348, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7352, -7356, -7340, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7360, -7364, -6696, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7368, -7372, -7288, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7376, -7380, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7384, -7380, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7388, -7376, -7384, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7376, -7380, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7384, -7380, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7392, -7376, -7384, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7396, -7400, -7392, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7376, -7380, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7384, -7380, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7404, -7376, -7384, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7408, -7412, -7404, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7416, -7380, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7420, -2252, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7424, -7428, -6856, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7432, -7436, -7440, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7444, -7448, -7452, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7456, -7460, -7408, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7464, -7468, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7472, -7468, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7476, -7464, -7472, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7464, -7468, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7472, -7468, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7480, -7464, -7472, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7484, -7488, -7480, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7464, -7468, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7472, -7468, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7492, -7464, -7472, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7496, -7500, -7492, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7504, -7300, -7032, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7508, -7512, -7516, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7520, -7524, -7508, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7528, -7532, -6864, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7536, -7540, -7456, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7544, -7548, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7552, -7548, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7556, -7544, -7552, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7544, -7548, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7552, -7548, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7560, -7544, -7552, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7564, -7568, -7560, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7544, -7548, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7552, -7548, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7572, -7544, -7552, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7576, -7580, -7572, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7584, -7548, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7588, -2364, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7592, -7596, -7024, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7600, -7604, -7608, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7612, -7616, -7620, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7624, -7628, -7576, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7632, -7636, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7640, -7636, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7644, -7632, -7640, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7632, -7636, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7640, -7636, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7648, -7632, -7640, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7652, -7656, -7648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7632, -7636, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7640, -7636, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7660, -7632, -7640, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7664, -7668, -7660, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7672, -7468, -7200, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7676, -7680, -7684, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7688, -7692, -7676, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7696, -7700, -7032, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7704, -7708, -7624, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7712, -7716, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7720, -7716, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7724, -7712, -7720, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7712, -7716, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7720, -7716, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7728, -7712, -7720, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7732, -7736, -7728, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7712, -7716, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7720, -7716, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7740, -7712, -7720, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7744, -7748, -7740, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7752, -7716, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7756, -2472, 310598401, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7760, -7764, -7192, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7768, -7772, -7776, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7780, -7784, -7788, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7792, -7796, -7744, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7800, -7804, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7808, -7804, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7812, -7800, -7808, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7800, -7804, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7808, -7804, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7816, -7800, -7808, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7820, -7824, -7816, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7800, -7804, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7808, -7804, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7828, -7800, -7808, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7832, -7836, -7828, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7840, -7636, -7368, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7844, -7848, -7852, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7856, -7860, -7844, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7864, -7868, -7200, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7872, -7876, -7792, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7880, -7884, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7888, -7884, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7892, -7880, -7888, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7880, -7884, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7888, -7884, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7896, -7880, -7888, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7900, -7904, -7896, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7880, -7884, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7888, -7884, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7908, -7880, -7888, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7912, -7916, -7908, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7920, -7884, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7924, -2580, 607225278, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7928, -7932, -7360, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7936, -7940, -7944, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7948, -7952, -7956, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-7960, -7964, -7912, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7968, -7972, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7976, -7972, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7980, -7968, -7976, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7968, -7972, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7976, -7972, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7984, -7968, -7976, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-7988, -7992, -7984, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-7968, -7972, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-7976, -7972, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-7996, -7968, -7976, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8000, -8004, -7996, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8008, -7804, -7536, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8012, -8016, -8020, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8024, -8028, -8012, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8032, -8036, -7368, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8040, -8044, -7960, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8048, -8052, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8056, -8052, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8060, -8048, -8056, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8048, -8052, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8056, -8052, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8064, -8048, -8056, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8068, -8072, -8064, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8048, -8052, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8056, -8052, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8076, -8048, -8056, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8080, -8084, -8076, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8088, -8052, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8092, -2684, 1426881987, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8096, -8100, -7528, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8104, -8108, -8112, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8116, -8120, -8124, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8128, -8132, -8080, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8136, -8140, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8144, -8140, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8148, -8136, -8144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8136, -8140, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8144, -8140, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8152, -8136, -8144, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8156, -8160, -8152, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8136, -8140, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8144, -8140, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8164, -8136, -8144, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8168, -8172, -8164, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8176, -7972, -7704, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8180, -8184, -8188, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8192, -8196, -8180, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8200, -8204, -7536, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8208, -8212, -8128, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8216, -8220, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8224, -8220, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8228, -8216, -8224, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8216, -8220, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8224, -8220, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8232, -8216, -8224, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8236, -8240, -8232, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8216, -8220, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8224, -8220, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8244, -8216, -8224, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8248, -8252, -8244, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8256, -8220, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8260, -2788, 1925078388, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8264, -8268, -7696, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8272, -8276, -8280, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8284, -8288, -8292, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8296, -8300, -8248, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8304, -8308, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8312, -8308, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8316, -8304, -8312, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8304, -8308, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8312, -8308, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8320, -8304, -8312, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8324, -8328, -8320, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8304, -8308, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8312, -8308, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8332, -8304, -8312, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8336, -8340, -8332, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8344, -8140, -7872, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8348, -8352, -8356, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8360, -8364, -8348, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8368, -8372, -7704, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8376, -8380, -8296, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8384, -8388, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8392, -8388, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8396, -8384, -8392, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8384, -8388, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8392, -8388, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8400, -8384, -8392, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8404, -8408, -8400, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8384, -8388, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8392, -8388, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8412, -8384, -8392, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8416, -8420, -8412, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8424, -8388, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8428, -2892, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8432, -8436, -7864, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8440, -8444, -8448, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8452, -8456, -8460, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8464, -8468, -8416, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8472, -8476, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8480, -8476, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8484, -8472, -8480, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8472, -8476, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8480, -8476, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8488, -8472, -8480, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8492, -8496, -8488, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8472, -8476, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8480, -8476, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8500, -8472, -8480, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8504, -8508, -8500, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8512, -8308, -8040, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8516, -8520, -8524, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8528, -8532, -8516, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8536, -8540, -7872, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8544, -8548, -8464, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8552, -8556, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8560, -8556, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8564, -8552, -8560, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8552, -8556, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8560, -8556, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8568, -8552, -8560, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8572, -8576, -8568, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8552, -8556, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8560, -8556, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8580, -8552, -8560, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8584, -8588, -8580, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8592, -8556, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8596, -1540, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8600, -8604, -8032, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8608, -8612, -8616, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8620, -8624, -8628, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8632, -8636, -8584, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8640, -8644, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8648, -8644, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8652, -8640, -8648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8640, -8644, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8648, -8644, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8656, -8640, -8648, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8660, -8664, -8656, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8640, -8644, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8648, -8644, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8668, -8640, -8648, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8672, -8676, -8668, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8680, -8476, -8208, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8684, -8688, -8692, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8696, -8700, -8684, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8704, -8708, -8040, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8712, -8716, -8632, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8720, -8724, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8728, -8724, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8732, -8720, -8728, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8720, -8724, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8728, -8724, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8736, -8720, -8728, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8740, -8744, -8736, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8720, -8724, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8728, -8724, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8748, -8720, -8728, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8752, -8756, -8748, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8760, -8724, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8764, -1652, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8768, -8772, -8200, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8776, -8780, -8784, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8788, -8792, -8796, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8800, -8804, -8752, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8808, -8812, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8816, -8812, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8820, -8808, -8816, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8808, -8812, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8816, -8812, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8824, -8808, -8816, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8828, -8832, -8824, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8808, -8812, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8816, -8812, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8836, -8808, -8816, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8840, -8844, -8836, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8848, -8644, -8376, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8852, -8856, -8860, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8864, -8868, -8852, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8872, -8876, -8208, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8880, -8884, -8800, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8888, -8892, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8896, -8892, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8900, -8888, -8896, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8888, -8892, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8896, -8892, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8904, -8888, -8896, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8908, -8912, -8904, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8888, -8892, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8896, -8892, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8916, -8888, -8896, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8920, -8924, -8916, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8928, -8892, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8932, -1764, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8936, -8940, -8368, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8944, -8948, -8952, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8956, -8960, -8964, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-8968, -8972, -8920, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8976, -8980, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8984, -8980, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8988, -8976, -8984, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8976, -8980, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8984, -8980, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-8992, -8976, -8984, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-8996, -9000, -8992, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-8976, -8980, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-8984, -8980, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9004, -8976, -8984, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9008, -9012, -9004, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9016, -8812, -8544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9020, -9024, -9028, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9032, -9036, -9020, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9040, -9044, -8376, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9048, -9052, -8968, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9056, -9060, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9064, -9060, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9068, -9056, -9064, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9056, -9060, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9064, -9060, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9072, -9056, -9064, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9076, -9080, -9072, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9056, -9060, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9064, -9060, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9084, -9056, -9064, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9088, -9092, -9084, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9096, -9060, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9100, -1876, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9104, -9108, -8536, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9112, -9116, -9120, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9124, -9128, -9132, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9136, -9140, -9088, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9144, -9148, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9152, -9148, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9156, -9144, -9152, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9144, -9148, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9152, -9148, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9160, -9144, -9152, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9164, -9168, -9160, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9144, -9148, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9152, -9148, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9172, -9144, -9152, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9176, -9180, -9172, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9184, -8980, -8712, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9188, -9192, -9196, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9200, -9204, -9188, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9208, -9212, -8544, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9216, -9220, -9136, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9224, -9228, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9232, -9228, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9236, -9224, -9232, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9224, -9228, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9232, -9228, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9240, -9224, -9232, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9244, -9248, -9240, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9224, -9228, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9232, -9228, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9252, -9224, -9232, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9256, -9260, -9252, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9264, -9228, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9268, -1988, 264347078, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9272, -9276, -8704, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9280, -9284, -9288, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9292, -9296, -9300, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9304, -9308, -9256, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9312, -9316, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9320, -9316, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9324, -9312, -9320, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9312, -9316, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9320, -9316, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9328, -9312, -9320, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9332, -9336, -9328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9312, -9316, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9320, -9316, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9340, -9312, -9320, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9344, -9348, -9340, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9352, -9148, -8880, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9356, -9360, -9364, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9368, -9372, -9356, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9376, -9380, -8712, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9384, -9388, -9304, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9392, -9396, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9400, -9396, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9404, -9392, -9400, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9392, -9396, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9400, -9396, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9408, -9392, -9400, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9412, -9416, -9408, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9392, -9396, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9400, -9396, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9420, -9392, -9400, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9424, -9428, -9420, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9432, -9396, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9436, -2100, 604807628, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9440, -9444, -8872, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9448, -9452, -9456, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9460, -9464, -9468, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9472, -9476, -9424, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9480, -9484, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9488, -9484, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9492, -9480, -9488, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9480, -9484, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9488, -9484, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9496, -9480, -9488, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9500, -9504, -9496, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9480, -9484, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9488, -9484, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9508, -9480, -9488, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9512, -9516, -9508, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9520, -9316, -9048, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9524, -9528, -9532, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9536, -9540, -9524, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9544, -9548, -8880, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9552, -9556, -9472, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9560, -9564, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9568, -9564, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9572, -9560, -9568, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9560, -9564, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9568, -9564, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9576, -9560, -9568, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9580, -9584, -9576, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9560, -9564, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9568, -9564, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9588, -9560, -9568, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9592, -9596, -9588, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9600, -9564, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9604, -2212, 770255983, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9608, -9612, -9040, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9616, -9620, -9624, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9628, -9632, -9636, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9640, -9644, -9592, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9648, -9652, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9656, -9652, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9660, -9648, -9656, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9648, -9652, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9656, -9652, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9664, -9648, -9656, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9668, -9672, -9664, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9648, -9652, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9656, -9652, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9676, -9648, -9656, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9680, -9684, -9676, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9688, -9484, -9216, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9692, -9696, -9700, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9704, -9708, -9692, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9712, -9716, -9048, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9720, -9724, -9640, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9728, -9732, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9736, -9732, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9740, -9728, -9736, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9728, -9732, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9736, -9732, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9744, -9728, -9736, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9748, -9752, -9744, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9728, -9732, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9736, -9732, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9756, -9728, -9736, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9760, -9764, -9756, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9768, -9732, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9772, -2324, 1249150122, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9776, -9780, -9208, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9784, -9788, -9792, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9796, -9800, -9804, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9808, -9812, -9760, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9816, -9820, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9824, -9820, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9828, -9816, -9824, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9816, -9820, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9824, -9820, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9832, -9816, -9824, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9836, -9840, -9832, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9816, -9820, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9824, -9820, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9844, -9816, -9824, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9848, -9852, -9844, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9856, -9652, -9384, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9860, -9864, -9868, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9872, -9876, -9860, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9880, -9884, -9216, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9888, -9892, -9808, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9896, -9900, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9904, -9900, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9908, -9896, -9904, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9896, -9900, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9904, -9900, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9912, -9896, -9904, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9916, -9920, -9912, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9896, -9900, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9904, -9900, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9924, -9896, -9904, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9928, -9932, -9924, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-9936, -9900, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9940, -2432, 1555081692, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9944, -9948, -9376, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9952, -9956, -9960, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9964, -9968, -9972, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-9976, -9980, -9928, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9984, -9988, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9992, -9988, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-9996, -9984, -9992, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9984, -9988, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9992, -9988, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10000, -9984, -9992, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10004, -10008, -10000, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-9984, -9988, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-9992, -9988, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10012, -9984, -9992, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10016, -10020, -10012, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10024, -9820, -9552, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10028, -10032, -10036, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10040, -10044, -10028, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10048, -10052, -9384, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10056, -10060, -9976, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10064, -10068, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10072, -10068, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10076, -10064, -10072, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10064, -10068, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10072, -10068, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10080, -10064, -10072, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10084, -10088, -10080, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10064, -10068, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10072, -10068, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10092, -10064, -10072, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10096, -10100, -10092, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10104, -10068, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10108, -2540, 1996064986, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10112, -10116, -9544, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10120, -10124, -10128, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10132, -10136, -10140, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10144, -10148, -10096, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10152, -10156, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10160, -10156, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10164, -10152, -10160, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10152, -10156, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10160, -10156, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10168, -10152, -10160, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10172, -10176, -10168, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10152, -10156, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10160, -10156, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10180, -10152, -10160, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10184, -10188, -10180, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10192, -9988, -9720, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10196, -10200, -10204, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10208, -10212, -10196, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10216, -10220, -9552, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10224, -10228, -10144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10232, -10236, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10240, -10236, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10244, -10232, -10240, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10232, -10236, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10240, -10236, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10248, -10232, -10240, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10252, -10256, -10248, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10232, -10236, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10240, -10236, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10260, -10232, -10240, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10264, -10268, -10260, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10272, -10236, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10276, -2644, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10280, -10284, -9712, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10288, -10292, -10296, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10300, -10304, -10308, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10312, -10316, -10264, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10320, -10324, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10328, -10324, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10332, -10320, -10328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10320, -10324, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10328, -10324, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10336, -10320, -10328, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10340, -10344, -10336, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10320, -10324, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10328, -10324, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10348, -10320, -10328, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10352, -10356, -10348, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10360, -10156, -9888, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10364, -10368, -10372, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10376, -10380, -10364, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10384, -10388, -9720, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10392, -10396, -10312, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10400, -10404, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10408, -10404, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10412, -10400, -10408, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10400, -10404, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10408, -10404, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10416, -10400, -10408, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10420, -10424, -10416, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10400, -10404, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10408, -10404, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10428, -10400, -10408, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10432, -10436, -10428, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10440, -10404, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10444, -2748, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10448, -10452, -9880, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10456, -10460, -10464, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10468, -10472, -10476, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10480, -10484, -10432, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10488, -10492, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10496, -10492, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10500, -10488, -10496, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10488, -10492, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10496, -10492, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10504, -10488, -10496, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10508, -10512, -10504, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10488, -10492, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10496, -10492, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10516, -10488, -10496, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10520, -10524, -10516, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10528, -10324, -10056, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10532, -10536, -10540, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10544, -10548, -10532, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10552, -10556, -9888, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10560, -10564, -10480, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10568, -10572, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10576, -10572, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10580, -10568, -10576, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10568, -10572, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10576, -10572, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10584, -10568, -10576, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10588, -10592, -10584, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10568, -10572, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10576, -10572, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10596, -10568, -10576, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10600, -10604, -10596, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10608, -10572, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10612, -2852, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10616, -10620, -10048, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10624, -10628, -10632, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10636, -10640, -10644, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10648, -10652, -10600, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10656, -10660, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10664, -10660, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10668, -10656, -10664, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10656, -10660, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10664, -10660, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10672, -10656, -10664, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10676, -10680, -10672, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10656, -10660, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10664, -10660, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10684, -10656, -10664, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10688, -10692, -10684, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10696, -10492, -10224, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10700, -10704, -10708, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10712, -10716, -10700, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10720, -10724, -10056, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10728, -10732, -10648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10736, -10740, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10744, -10740, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10748, -10736, -10744, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10736, -10740, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10744, -10740, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10752, -10736, -10744, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10756, -10760, -10752, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10736, -10740, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10744, -10740, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10764, -10736, -10744, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10768, -10772, -10764, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10776, -10740, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10780, -2956, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10784, -10788, -10216, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10792, -10796, -10800, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10804, -10808, -10812, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10816, -10820, -10768, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10824, -10828, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10832, -10828, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10836, -10824, -10832, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10824, -10828, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10832, -10828, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10840, -10824, -10832, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10844, -10848, -10840, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10824, -10828, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10832, -10828, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10852, -10824, -10832, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10856, -10860, -10852, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10864, -10660, -10392, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10868, -10872, -10876, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10880, -10884, -10868, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10888, -10892, -10224, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10896, -10900, -10816, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10904, -10908, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10912, -10908, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10916, -10904, -10912, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10904, -10908, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10912, -10908, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10920, -10904, -10912, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10924, -10928, -10920, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10904, -10908, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-10912, -10908, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-10932, -10904, -10912, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10936, -10940, -10932, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-10944, -10908, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10948, -3048, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10952, -10956, -10384, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10960, -10964, -10968, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10972, -10976, -10980, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-10984, -10988, -10936, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10992, -10996, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11000, -10996, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11004, -10992, -11000, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10992, -10996, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11000, -10996, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11008, -10992, -11000, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11012, -11016, -11008, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-10992, -10996, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11000, -10996, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11020, -10992, -11000, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11024, -11028, -11020, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11032, -10828, -10560, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11036, -11040, -11044, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11048, -11052, -11036, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11056, -11060, -10392, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11064, -11068, -10984, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11072, -11076, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11080, -11076, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11084, -11072, -11080, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11072, -11076, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11080, -11076, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11088, -11072, -11080, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11092, -11096, -11088, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11072, -11076, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11080, -11076, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11100, -11072, -11080, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11104, -11108, -11100, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11112, -11076, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11116, -3140, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11120, -11124, -10552, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11128, -11132, -11136, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11140, -11144, -11148, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11152, -11156, -11104, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11160, -11164, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11168, -11164, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11172, -11160, -11168, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11160, -11164, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11168, -11164, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11176, -11160, -11168, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11180, -11184, -11176, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11160, -11164, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11168, -11164, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11188, -11160, -11168, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11192, -11196, -11188, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11200, -10996, -10728, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11204, -11208, -11212, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11216, -11220, -11204, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11224, -11228, -10560, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11232, -11236, -11152, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11240, -11244, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11248, -11244, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11252, -11240, -11248, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11240, -11244, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11248, -11244, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11256, -11240, -11248, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11260, -11264, -11256, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11240, -11244, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11248, -11244, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11268, -11240, -11248, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11272, -11276, -11268, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11280, -11244, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11284, -3232, 113926993, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11288, -11292, -10720, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11296, -11300, -11304, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11308, -11312, -11316, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11320, -11324, -11272, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11328, -11332, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11336, -11332, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11340, -11328, -11336, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11328, -11332, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11336, -11332, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11344, -11328, -11336, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11348, -11352, -11344, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11328, -11332, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11336, -11332, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11356, -11328, -11336, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11360, -11364, -11356, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11368, -11164, -10896, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11372, -11376, -11380, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11384, -11388, -11372, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11392, -11396, -10728, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11400, -11404, -11320, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11408, -11412, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11416, -11412, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11420, -11408, -11416, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11408, -11412, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11416, -11412, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11424, -11408, -11416, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11428, -11432, -11424, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11408, -11412, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11416, -11412, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11436, -11408, -11416, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11440, -11444, -11436, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11448, -11412, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11452, -3324, 338241895, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11456, -11460, -10888, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11464, -11468, -11472, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11476, -11480, -11484, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11488, -11492, -11440, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11496, -11500, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11504, -11500, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11508, -11496, -11504, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11496, -11500, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11504, -11500, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11512, -11496, -11504, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11516, -11520, -11512, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11496, -11500, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11504, -11500, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11524, -11496, -11504, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11528, -11532, -11524, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11536, -11332, -11064, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11540, -11544, -11548, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11552, -11556, -11540, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11560, -11564, -10896, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11568, -11572, -11488, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11576, -11580, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11584, -11580, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11588, -11576, -11584, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11576, -11580, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11584, -11580, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11592, -11576, -11584, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11596, -11600, -11592, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11576, -11580, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11584, -11580, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11604, -11576, -11584, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11608, -11612, -11604, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11616, -11580, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11620, -3416, 666307205, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11624, -11628, -11056, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11632, -11636, -11640, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11644, -11648, -11652, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11656, -11660, -11608, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11664, -11668, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11672, -11668, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11676, -11664, -11672, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11664, -11668, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11672, -11668, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11680, -11664, -11672, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11684, -11688, -11680, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11664, -11668, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11672, -11668, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11692, -11664, -11672, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11696, -11700, -11692, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11704, -11500, -11232, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11708, -11712, -11716, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11720, -11724, -11708, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11728, -11732, -11064, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11736, -11740, -11656, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11744, -11748, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11752, -11748, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11756, -11744, -11752, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11744, -11748, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11752, -11748, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11760, -11744, -11752, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11764, -11768, -11760, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11744, -11748, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11752, -11748, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11772, -11744, -11752, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11776, -11780, -11772, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11784, -11748, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11788, -3508, 773529912, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11792, -11796, -11224, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11800, -11804, -11808, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11812, -11816, -11820, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11824, -11828, -11776, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11832, -11836, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11840, -11836, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11844, -11832, -11840, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11832, -11836, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11840, -11836, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11848, -11832, -11840, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11852, -11856, -11848, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11832, -11836, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11840, -11836, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11860, -11832, -11840, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11864, -11868, -11860, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11872, -11668, -11400, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11876, -11880, -11884, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11888, -11892, -11876, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11896, -11900, -11232, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11904, -11908, -11824, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11912, -11916, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11920, -11916, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11924, -11912, -11920, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11912, -11916, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11920, -11916, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11928, -11912, -11920, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11932, -11936, -11928, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-11912, -11916, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-11920, -11916, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-11940, -11912, -11920, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11944, -11948, -11940, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-11952, -11916, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11956, -3600, 1294757372, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11960, -11964, -11392, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11968, -11972, -11976, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11980, -11984, -11988, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-11992, -11996, -11944, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12000, -12004, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12008, -12004, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12012, -12000, -12008, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12000, -12004, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12008, -12004, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12016, -12000, -12008, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12020, -12024, -12016, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12000, -12004, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12008, -12004, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12028, -12000, -12008, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12032, -12036, -12028, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12040, -11836, -11568, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12044, -12048, -12052, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12056, -12060, -12044, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12064, -12068, -11400, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12072, -12076, -11992, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12080, -12084, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12088, -12084, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12092, -12080, -12088, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12080, -12084, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12088, -12084, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12096, -12080, -12088, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12100, -12104, -12096, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12080, -12084, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12088, -12084, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12108, -12080, -12088, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12112, -12116, -12108, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12120, -12084, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12124, -3692, 1396182291, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12128, -12132, -11560, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12136, -12140, -12144, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12148, -12152, -12156, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12160, -12164, -12112, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12168, -12172, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12176, -12172, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12180, -12168, -12176, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12168, -12172, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12176, -12172, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12184, -12168, -12176, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12188, -12192, -12184, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12168, -12172, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12176, -12172, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12196, -12168, -12176, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12200, -12204, -12196, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12208, -12004, -11736, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12212, -12216, -12220, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12224, -12228, -12212, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12232, -12236, -11568, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12240, -12244, -12160, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12248, -12252, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12256, -12252, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12260, -12248, -12256, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12248, -12252, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12256, -12252, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12264, -12248, -12256, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12268, -12272, -12264, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12248, -12252, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12256, -12252, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12276, -12248, -12256, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12280, -12284, -12276, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12288, -12252, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12292, -3784, 1695183700, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12296, -12300, -11728, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12304, -12308, -12312, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12316, -12320, -12324, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12328, -12332, -12280, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12336, -12340, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12344, -12340, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12348, -12336, -12344, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12336, -12340, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12344, -12340, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12352, -12336, -12344, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12356, -12360, -12352, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12336, -12340, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12344, -12340, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12364, -12336, -12344, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12368, -12372, -12364, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12376, -12172, -11904, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12380, -12384, -12388, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12392, -12396, -12380, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12400, -12404, -11736, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12408, -12412, -12328, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12416, -12420, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12424, -12420, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12428, -12416, -12424, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12416, -12420, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12424, -12420, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12432, -12416, -12424, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12436, -12440, -12432, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12416, -12420, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12424, -12420, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12444, -12416, -12424, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12448, -12452, -12444, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12456, -12420, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12460, -3876, 1986661051, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12464, -12468, -11896, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12472, -12476, -12480, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12484, -12488, -12492, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12496, -12500, -12448, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12504, -12508, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12512, -12508, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12516, -12504, -12512, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12504, -12508, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12512, -12508, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12520, -12504, -12512, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12524, -12528, -12520, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12504, -12508, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12512, -12508, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12532, -12504, -12512, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12536, -12540, -12532, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12544, -12340, -12072, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12548, -12552, -12556, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12560, -12564, -12548, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12568, -12572, -11904, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12576, -12580, -12496, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12584, -12588, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12592, -12588, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12596, -12584, -12592, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12584, -12588, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12592, -12588, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12600, -12584, -12592, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12604, -12608, -12600, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12584, -12588, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12592, -12588, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12612, -12584, -12592, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12616, -12620, -12612, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12624, -12588, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12628, -3968, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12632, -12636, -12064, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12640, -12644, -12648, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12652, -12656, -12660, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12664, -12668, -12616, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12672, -12676, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12680, -12676, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12684, -12672, -12680, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12672, -12676, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12680, -12676, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12688, -12672, -12680, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12692, -12696, -12688, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12672, -12676, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12680, -12676, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12700, -12672, -12680, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12704, -12708, -12700, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12712, -12508, -12240, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12716, -12720, -12724, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12728, -12732, -12716, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12736, -12740, -12072, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12744, -12748, -12664, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12752, -12756, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12760, -12756, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12764, -12752, -12760, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12752, -12756, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12760, -12756, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12768, -12752, -12760, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12772, -12776, -12768, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12752, -12756, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12760, -12756, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12780, -12752, -12760, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12784, -12788, -12780, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12792, -12756, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12796, -4060, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12800, -12804, -12232, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12808, -12812, -12816, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12820, -12824, -12828, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12832, -12836, -12784, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12840, -12844, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12848, -12844, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12852, -12840, -12848, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12840, -12844, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12848, -12844, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12856, -12840, -12848, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12860, -12864, -12856, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12840, -12844, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12848, -12844, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12868, -12840, -12848, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12872, -12876, -12868, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12880, -12676, -12408, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12884, -12888, -12892, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12896, -12900, -12884, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12904, -12908, -12240, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12912, -12916, -12832, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12920, -12924, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12928, -12924, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12932, -12920, -12928, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12920, -12924, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12928, -12924, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12936, -12920, -12928, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12940, -12944, -12936, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-12920, -12924, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-12928, -12924, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-12948, -12920, -12928, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12952, -12956, -12948, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-12960, -12924, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12964, -4152, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12968, -12972, -12400, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12976, -12980, -12984, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-12988, -12992, -12996, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13000, -13004, -12952, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13008, -13012, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13016, -13012, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13020, -13008, -13016, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13008, -13012, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13016, -13012, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13024, -13008, -13016, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13028, -13032, -13024, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13008, -13012, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13016, -13012, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13036, -13008, -13016, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13040, -13044, -13036, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13048, -12844, -12576, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13052, -13056, -13060, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13064, -13068, -13052, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13072, -13076, -12408, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13080, -13084, -13000, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13088, -13092, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13096, -13092, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13100, -13088, -13096, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13088, -13092, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13096, -13092, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13104, -13088, -13096, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13108, -13112, -13104, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13088, -13092, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13096, -13092, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13116, -13088, -13096, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13120, -13124, -13116, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13128, -13092, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13132, -4244, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13136, -13140, -12568, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13144, -13148, -13152, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13156, -13160, -13164, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13168, -13172, -13120, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13176, -13180, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13184, -13180, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13188, -13176, -13184, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13176, -13180, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13184, -13180, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13192, -13176, -13184, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13196, -13200, -13192, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13176, -13180, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13184, -13180, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13204, -13176, -13184, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13208, -13212, -13204, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13216, -13012, -12744, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13220, -13224, -13228, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13232, -13236, -13220, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13240, -13244, -12576, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13248, -13252, -13168, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13256, -13260, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13264, -13260, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13268, -13256, -13264, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13256, -13260, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13264, -13260, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13272, -13256, -13264, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13276, -13280, -13272, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13256, -13260, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13264, -13260, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13284, -13256, -13264, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13288, -13292, -13284, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13296, -13260, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13300, -4336, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13304, -13308, -12736, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13312, -13316, -13320, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13324, -13328, -13332, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13336, -13340, -13288, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13344, -13348, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13352, -13348, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13356, -13344, -13352, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13344, -13348, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13352, -13348, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13360, -13344, -13352, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13364, -13368, -13360, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13344, -13348, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13352, -13348, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13372, -13344, -13352, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13376, -13380, -13372, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13384, -13180, -12912, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13388, -13392, -13396, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13400, -13404, -13388, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13408, -13412, -12744, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13416, -13420, -13336, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13424, -13428, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13432, -13428, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13436, -13424, -13432, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13424, -13428, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13432, -13428, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13440, -13424, -13432, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13444, -13448, -13440, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13424, -13428, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13432, -13428, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13452, -13424, -13432, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13456, -13460, -13452, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13464, -13428, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13468, -4428, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13472, -13476, -12904, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13480, -13484, -13488, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13492, -13496, -13500, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13504, -13508, -13456, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13512, -13516, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13520, -13516, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13524, -13512, -13520, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13512, -13516, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13520, -13516, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13528, -13512, -13520, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13532, -13536, -13528, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13512, -13516, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13520, -13516, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13540, -13512, -13520, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13544, -13548, -13540, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13552, -13348, -13080, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13556, -13560, -13564, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13568, -13572, -13556, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13576, -13580, -12912, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13584, -13588, -13504, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13592, -13596, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13600, -13596, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13604, -13592, -13600, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13592, -13596, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13600, -13596, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13608, -13592, -13600, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13612, -13616, -13608, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13592, -13596, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13600, -13596, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13620, -13592, -13600, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13624, -13628, -13620, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13632, -13596, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13636, -4520, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13640, -13644, -13072, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13648, -13652, -13656, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13660, -13664, -13668, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13672, -13676, -13624, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13680, -13684, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13688, -13684, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13692, -13680, -13688, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13680, -13684, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13688, -13684, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13696, -13680, -13688, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13700, -13704, -13696, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13680, -13684, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13688, -13684, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13708, -13680, -13688, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13712, -13716, -13708, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13720, -13516, -13248, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13724, -13728, -13732, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13736, -13740, -13724, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13744, -13748, -13080, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13752, -13756, -13672, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13760, -13764, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13768, -13764, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13772, -13760, -13768, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13760, -13764, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13768, -13764, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13776, -13760, -13768, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13780, -13784, -13776, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13760, -13764, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13768, -13764, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13788, -13760, -13768, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13792, -13796, -13788, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13800, -13764, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13804, -4612, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13808, -13812, -13240, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13816, -13820, -13824, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13828, -13832, -13836, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13840, -13844, -13792, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13848, -13852, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13856, -13852, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13860, -13848, -13856, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13848, -13852, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13856, -13852, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13864, -13848, -13856, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13868, -13872, -13864, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13848, -13852, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13856, -13852, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13876, -13848, -13856, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13880, -13884, -13876, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13888, -13684, -13416, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13892, -13896, -13900, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13904, -13908, -13892, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13912, -13916, -13248, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13920, -13924, -13840, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13928, -13932, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13936, -13932, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13940, -13928, -13936, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13928, -13932, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13936, -13932, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13944, -13928, -13936, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13948, -13952, -13944, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-13928, -13932, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-13936, -13932, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-13956, -13928, -13936, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13960, -13964, -13956, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-13968, -13932, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13972, -4704, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13976, -13980, -13408, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13984, -13988, -13992, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-13996, -14000, -14004, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14008, -14012, -13960, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14016, -14020, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14024, -14020, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14028, -14016, -14024, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14016, -14020, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14024, -14020, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14032, -14016, -14024, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14036, -14040, -14032, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14016, -14020, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14024, -14020, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14044, -14016, -14024, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14048, -14052, -14044, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14056, -13852, -13584, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14060, -14064, -14068, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14072, -14076, -14060, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14080, -14084, -13416, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14088, -14092, -14008, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14096, -14100, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14104, -14100, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14108, -14096, -14104, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14096, -14100, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14104, -14100, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14112, -14096, -14104, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14116, -14120, -14112, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14096, -14100, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14104, -14100, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14124, -14096, -14104, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14128, -14132, -14124, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14136, -14100, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14140, -4796, 275423344, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14144, -14148, -13576, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14152, -14156, -14160, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14164, -14168, -14172, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14176, -14180, -14128, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14184, -14188, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14192, -14188, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14196, -14184, -14192, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14184, -14188, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14192, -14188, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14200, -14184, -14192, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14204, -14208, -14200, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14184, -14188, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14192, -14188, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14212, -14184, -14192, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14216, -14220, -14212, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14224, -14020, -13752, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14228, -14232, -14236, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14240, -14244, -14228, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14248, -14252, -13584, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14256, -14260, -14176, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14264, -14268, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14272, -14268, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14276, -14264, -14272, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14264, -14268, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14272, -14268, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14280, -14264, -14272, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14284, -14288, -14280, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14264, -14268, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14272, -14268, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14292, -14264, -14272, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14296, -14300, -14292, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14304, -14268, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14308, -4888, 430227734, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14312, -14316, -13744, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14320, -14324, -14328, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14332, -14336, -14340, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14344, -14348, -14296, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14352, -14356, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14360, -14356, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14364, -14352, -14360, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14352, -14356, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14360, -14356, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14368, -14352, -14360, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14372, -14376, -14368, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14352, -14356, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14360, -14356, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14380, -14352, -14360, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14384, -14388, -14380, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14392, -14188, -13920, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14396, -14400, -14404, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14408, -14412, -14396, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14416, -14420, -13752, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14424, -14428, -14344, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14432, -14436, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14440, -14436, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14444, -14432, -14440, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14432, -14436, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14440, -14436, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14448, -14432, -14440, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14452, -14456, -14448, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14432, -14436, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14440, -14436, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14460, -14432, -14440, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14464, -14468, -14460, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14472, -14436, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14476, -4980, 506948616, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14480, -14484, -13912, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14488, -14492, -14496, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14500, -14504, -14508, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14512, -14516, -14464, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14520, -14524, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14528, -14524, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14532, -14520, -14528, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14520, -14524, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14528, -14524, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14536, -14520, -14528, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14540, -14544, -14536, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14520, -14524, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14528, -14524, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14548, -14520, -14528, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14552, -14556, -14548, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14560, -14356, -14088, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14564, -14568, -14572, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14576, -14580, -14564, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14584, -14588, -13920, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14592, -14596, -14512, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14600, -14604, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14608, -14604, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14612, -14600, -14608, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14600, -14604, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14608, -14604, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14616, -14600, -14608, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14620, -14624, -14616, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14600, -14604, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14608, -14604, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14628, -14600, -14608, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14632, -14636, -14628, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14640, -14604, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14644, -5072, 659060556, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14648, -14652, -14080, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14656, -14660, -14664, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14668, -14672, -14676, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14680, -14684, -14632, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14688, -14692, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14696, -14692, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14700, -14688, -14696, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14688, -14692, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14696, -14692, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14704, -14688, -14696, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14708, -14712, -14704, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14688, -14692, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14696, -14692, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14716, -14688, -14696, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14720, -14724, -14716, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14728, -14524, -14256, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14732, -14736, -14740, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14744, -14748, -14732, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14752, -14756, -14088, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14760, -14764, -14680, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14768, -14772, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14776, -14772, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14780, -14768, -14776, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14768, -14772, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14776, -14772, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14784, -14768, -14776, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14788, -14792, -14784, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14768, -14772, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14776, -14772, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14796, -14768, -14776, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14800, -14804, -14796, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14808, -14772, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14812, -5164, 883997877, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14816, -14820, -14248, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14824, -14828, -14832, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14836, -14840, -14844, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14848, -14852, -14800, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14856, -14860, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14864, -14860, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14868, -14856, -14864, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14856, -14860, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14864, -14860, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14872, -14856, -14864, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14876, -14880, -14872, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14856, -14860, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14864, -14860, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14884, -14856, -14864, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14888, -14892, -14884, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14896, -14692, -14424, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14900, -14904, -14908, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14912, -14916, -14900, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14920, -14924, -14256, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14928, -14932, -14848, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14936, -14940, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14944, -14940, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14948, -14936, -14944, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14936, -14940, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14944, -14940, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14952, -14936, -14944, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14956, -14960, -14952, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-14936, -14940, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-14944, -14940, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-14964, -14936, -14944, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14968, -14972, -14964, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-14976, -14940, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14980, -5256, 958139571, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14984, -14988, -14416, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-14992, -14996, -15000, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15004, -15008, -15012, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15016, -15020, -14968, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15024, -15028, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15032, -15028, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15036, -15024, -15032, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15024, -15028, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15032, -15028, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15040, -15024, -15032, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15044, -15048, -15040, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15024, -15028, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15032, -15028, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15052, -15024, -15032, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15056, -15060, -15052, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15064, -14860, -14592, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15068, -15072, -15076, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15080, -15084, -15068, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15088, -15092, -14424, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15096, -15100, -15016, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15104, -15108, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15112, -15108, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15116, -15104, -15112, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15104, -15108, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15112, -15108, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15120, -15104, -15112, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15124, -15128, -15120, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15104, -15108, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15112, -15108, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15132, -15104, -15112, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15136, -15140, -15132, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15144, -15108, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15148, -5348, 1322822218, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15152, -15156, -14584, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15160, -15164, -15168, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15172, -15176, -15180, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15184, -15188, -15136, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15192, -15196, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15200, -15196, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15204, -15192, -15200, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15192, -15196, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15200, -15196, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15208, -15192, -15200, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15212, -15216, -15208, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15192, -15196, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15200, -15196, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15220, -15192, -15200, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15224, -15228, -15220, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15232, -15028, -14760, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15236, -15240, -15244, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15248, -15252, -15236, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15256, -15260, -14592, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15264, -15268, -15184, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15272, -15276, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15280, -15276, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15284, -15272, -15280, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15272, -15276, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15280, -15276, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15288, -15272, -15280, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15292, -15296, -15288, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15272, -15276, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15280, -15276, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15300, -15272, -15280, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15304, -15308, -15300, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15312, -15276, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15316, -5440, 1537002063, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15320, -15324, -14752, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15328, -15332, -15336, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15340, -15344, -15348, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15352, -15356, -15304, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15360, -15364, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15368, -15364, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15372, -15360, -15368, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15360, -15364, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15368, -15364, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15376, -15360, -15368, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15380, -15384, -15376, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15360, -15364, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15368, -15364, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15388, -15360, -15368, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15392, -15396, -15388, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15400, -15196, -14928, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15404, -15408, -15412, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15416, -15420, -15404, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15424, -15428, -14760, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15432, -15436, -15352, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15440, -15444, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15448, -15444, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15452, -15440, -15448, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15440, -15444, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15448, -15444, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15456, -15440, -15448, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15460, -15464, -15456, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15440, -15444, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15448, -15444, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15468, -15440, -15448, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15472, -15476, -15468, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15480, -15444, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15484, -5532, 1747873779, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15488, -15492, -14920, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15496, -15500, -15504, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15508, -15512, -15516, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15520, -15524, -15472, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15528, -15532, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15536, -15532, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15540, -15528, -15536, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15528, -15532, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15536, -15532, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15544, -15528, -15536, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15548, -15552, -15544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15528, -15532, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15536, -15532, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15556, -15528, -15536, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15560, -15564, -15556, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15568, -15364, -15096, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15572, -15576, -15580, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15584, -15588, -15572, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15592, -15596, -14928, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15600, -15604, -15520, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15608, -15612, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15616, -15612, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15620, -15608, -15616, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15608, -15612, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15616, -15612, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15624, -15608, -15616, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15628, -15632, -15624, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15608, -15612, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15616, -15612, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15636, -15608, -15616, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15640, -15644, -15636, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15648, -15612, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15652, -5624, 1955562222, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15656, -15660, -15088, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15664, -15668, -15672, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15676, -15680, -15684, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15688, -15692, -15640, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15696, -15700, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15704, -15700, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15708, -15696, -15704, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15696, -15700, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15704, -15700, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15712, -15696, -15704, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15716, -15720, -15712, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15696, -15700, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15704, -15700, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15724, -15696, -15704, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15728, -15732, -15724, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15736, -15532, -15264, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15740, -15744, -15748, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15752, -15756, -15740, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15760, -15764, -15096, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15768, -15772, -15688, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15776, -15780, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15784, -15780, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15788, -15776, -15784, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15776, -15780, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15784, -15780, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15792, -15776, -15784, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15796, -15800, -15792, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15776, -15780, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15784, -15780, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15804, -15776, -15784, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15808, -15812, -15804, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15816, -15780, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15820, -5716, 2024104815, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15824, -15828, -15256, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15832, -15836, -15840, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15844, -15848, -15852, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15856, -15860, -15808, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15864, -15868, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15872, -15868, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15876, -15864, -15872, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15864, -15868, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15872, -15868, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15880, -15864, -15872, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15884, -15888, -15880, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15864, -15868, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15872, -15868, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15892, -15864, -15872, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15896, -15900, -15892, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15904, -15700, -15432, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15908, -15912, -15916, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15920, -15924, -15908, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15928, -15932, -15264, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15936, -15940, -15856, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15944, -15948, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15952, -15948, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15956, -15944, -15952, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15944, -15948, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15952, -15948, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15960, -15944, -15952, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15964, -15968, -15960, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-15944, -15948, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-15952, -15948, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-15972, -15944, -15952, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15976, -15980, -15972, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-15984, -15948, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15988, -5808, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-15992, -15996, -15424, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16000, -16004, -16008, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16012, -16016, -16020, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16024, -16028, -15976, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16032, -16036, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16040, -16036, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16044, -16032, -16040, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16032, -16036, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16040, -16036, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16048, -16032, -16040, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16052, -16056, -16048, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16032, -16036, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16040, -16036, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16060, -16032, -16040, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16064, -16068, -16060, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16072, -15868, -15600, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16076, -16080, -16084, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16088, -16092, -16076, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16096, -16100, -15432, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16104, -16108, -16024, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16112, -16116, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16120, -16116, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16124, -16112, -16120, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16112, -16116, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16120, -16116, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16128, -16112, -16120, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16132, -16136, -16128, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16112, -16116, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16120, -16116, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16140, -16112, -16120, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16144, -16148, -16140, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16152, -16116, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16156, -5900, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16160, -16164, -15592, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16168, -16172, -16176, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16180, -16184, -16188, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16192, -16196, -16144, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16200, -16204, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16208, -16204, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16212, -16200, -16208, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16200, -16204, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16208, -16204, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16216, -16200, -16208, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16220, -16224, -16216, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16200, -16204, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16208, -16204, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16228, -16200, -16208, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16232, -16236, -16228, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16240, -16036, -15768, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16244, -16248, -16252, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16256, -16260, -16244, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16264, -16268, -15600, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16272, -16276, -16192, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16280, -16284, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16288, -16284, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16292, -16280, -16288, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16280, -16284, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16288, -16284, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16296, -16280, -16288, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16300, -16304, -16296, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16280, -16284, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16288, -16284, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16308, -16280, -16288, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16312, -16316, -16308, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16320, -16284, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16324, -5992, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16328, -16332, -15760, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16336, -16340, -16344, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16348, -16352, -16356, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16360, -16364, -16312, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16368, -16372, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16376, -16372, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16380, -16368, -16376, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16368, -16372, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16376, -16372, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16384, -16368, -16376, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16388, -16392, -16384, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16368, -16372, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16376, -16372, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16396, -16368, -16376, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16400, -16404, -16396, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16408, -16204, -15936, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16412, -16416, -16420, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16424, -16428, -16412, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16432, -16436, -15768, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16440, -16444, -16360, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16448, -16452, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16456, -16452, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16460, -16448, -16456, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16448, -16452, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16456, -16452, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16464, -16448, -16456, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16468, -16472, -16464, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16448, -16452, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16456, -16452, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16476, -16448, -16456, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16480, -16484, -16476, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16488, -16452, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16492, -6060, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16496, -16500, -15928, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16504, -16508, -16512, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16516, -16520, -16524, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16528, -16532, -16480, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16536, -16540, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16544, -16540, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16548, -16536, -16544, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16536, -16540, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16544, -16540, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16552, -16536, -16544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16556, -16560, -16552, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16536, -16540, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16544, -16540, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16564, -16536, -16544, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16568, -16572, -16564, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16576, -16372, -16104, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16580, -16584, -16588, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16592, -16596, -16580, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16600, -16604, -15936, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16608, -16612, -16528, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16616, -16620, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16624, -16620, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16628, -16616, -16624, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16616, -16620, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16624, -16620, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16632, -16616, -16624, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16636, -16640, -16632, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16616, -16620, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16624, -16620, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16644, -16616, -16624, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16648, -16652, -16644, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16656, -16620, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16660, -4704, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16664, -16668, -6048, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16672, -16676, -5428, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16680, -16684, -6020, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16688, -16692, -16096, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16696, -16700, -16704, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16708, -16712, -16716, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16720, -16724, -16648, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16728, -16732, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16736, -16732, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16740, -16728, -16736, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16728, -16732, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16736, -16732, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16744, -16728, -16736, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16748, -16752, -16744, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16728, -16732, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16736, -16732, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16756, -16728, -16736, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16760, -16764, -16756, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16768, -16540, -16272, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16772, -16776, -16780, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16784, -16788, -16772, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16792, -16796, -16104, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16800, -16804, -16720, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16808, -16812, 26, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16816, -16812, 6, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16820, -16808, -16816, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16808, -16812, 21, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16816, -16812, 11, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16824, -16808, -16816, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16828, -16832, -16824, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16808, -16812, 7, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16816, -16812, 25, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16836, -16808, -16816, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16840, -16844, -16836, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16848, -16812, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16852, -4796, 0, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16856, -16860, -6116, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16864, -16868, -5520, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16872, -16876, -6088, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16880, -16884, -16264, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16888, -16892, -16896, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16900, -16904, -16908, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16912, -16916, -16840, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16920, -16924, 30, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16928, -16924, 2, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16932, -16920, -16928, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16920, -16924, 19, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16928, -16924, 13, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16936, -16920, -16928, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16940, -16944, -16936, 0, 0]),
        },
        InstructionWord {
            opcode: 105,
            operands: Operands([-16920, -16924, 10, 0, 0]),
        },
        InstructionWord {
            opcode: 106,
            operands: Operands([-16928, -16924, 22, 0, 0]),
        },
        InstructionWord {
            opcode: 108,
            operands: Operands([-16948, -16920, -16928, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16952, -16956, -16948, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16960, -16732, -16440, 0, 0]),
        },
        InstructionWord {
            opcode: 109,
            operands: Operands([-16964, -16968, -16972, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16976, -16980, -6124, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16984, -16988, -16952, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-16992, -16996, -16912, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17000, -16924, -6132, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17004, -16732, -6140, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17008, -16540, -6148, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17012, -16372, -6156, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17016, -17020, -16912, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17024, -16812, -6164, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17028, -16620, -6172, 0, 0]),
        },
        InstructionWord {
            opcode: 100,
            operands: Operands([-17032, -16452, -6180, 0, 0]),
        },
    ]);
    instructions
}
