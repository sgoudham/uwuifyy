pub const FACES_SIZE: usize = 106;

pub const FACES: [&[u8]; FACES_SIZE] = [
    b"OwO",
    b"UwU",
    b">w<",
    b"^w^",
    b"^-^",
    b":3",
    b"x3",
    b"xDD",
    b";;w;;",
    b">_<",
    b">_>",
    b"^.^",
    b":33",
    b"uWu",
    // (* ^ ω ^)
    kaomoji_ru::positive_emotions::JOY[0],
    // (´ ∀ ` *)
    kaomoji_ru::positive_emotions::JOY[1],
    // (o^▽^o)
    kaomoji_ru::positive_emotions::JOY[4],
    // (⌒▽⌒)☆
    kaomoji_ru::positive_emotions::JOY[5],
    // <(￣︶￣)>
    kaomoji_ru::positive_emotions::JOY[6],
    // ヽ(・∀・)ﾉ
    kaomoji_ru::positive_emotions::JOY[8],
    // (´｡• ω •｡`)
    kaomoji_ru::positive_emotions::JOY[9],
    // (￣ω￣)
    kaomoji_ru::positive_emotions::JOY[10],
    // (o･ω･o)
    kaomoji_ru::positive_emotions::JOY[12],
    // ヽ(*・ω・)ﾉ
    kaomoji_ru::positive_emotions::JOY[14],
    // (^人^)
    kaomoji_ru::positive_emotions::JOY[16],
    // (*´▽`*)
    kaomoji_ru::positive_emotions::JOY[18],
    // ( ´ ω ` )
    kaomoji_ru::positive_emotions::JOY[20],
    // (≧◡≦)
    kaomoji_ru::positive_emotions::JOY[22],
    // (o´∀`o)
    kaomoji_ru::positive_emotions::JOY[23],
    // (´• ω •`)
    kaomoji_ru::positive_emotions::JOY[24],
    // (＾▽＾)
    kaomoji_ru::positive_emotions::JOY[25],
    // (⌒ω⌒)
    kaomoji_ru::positive_emotions::JOY[26],
    // ╰(▔∀▔)╯
    kaomoji_ru::positive_emotions::JOY[28],
    // (*^‿^*)
    kaomoji_ru::positive_emotions::JOY[30],
    // (✯◡✯)
    kaomoji_ru::positive_emotions::JOY[32],
    // (*≧ω≦*)
    kaomoji_ru::positive_emotions::JOY[34],
    // (☆▽☆)
    kaomoji_ru::positive_emotions::JOY[35],
    // ＼(≧▽≦)／
    kaomoji_ru::positive_emotions::JOY[37],
    // ヽ(o＾▽＾o)ノ
    kaomoji_ru::positive_emotions::JOY[38],
    // (*°▽°*)
    kaomoji_ru::positive_emotions::JOY[40],
    // (✧ω✧)
    kaomoji_ru::positive_emotions::JOY[42],
    // ヽ(*⌒▽⌒*)ﾉ
    kaomoji_ru::positive_emotions::JOY[43],
    // ヽ(>∀<☆)ノ
    kaomoji_ru::positive_emotions::JOY[48],
    // o(≧▽≦)o
    kaomoji_ru::positive_emotions::JOY[49],
    // (☆ω☆)
    kaomoji_ru::positive_emotions::JOY[50],
    // (っ˘ω˘ς )
    kaomoji_ru::positive_emotions::JOY[51],
    // \(★ω★)/
    kaomoji_ru::positive_emotions::JOY[57],
    // (╯✧▽✧)╯
    kaomoji_ru::positive_emotions::JOY[60],
    // o(>ω<)o
    kaomoji_ru::positive_emotions::JOY[61],
    // (´･ᴗ･ ` )
    kaomoji_ru::positive_emotions::JOY[72],
    // (￢‿￢ )
    kaomoji_ru::positive_emotions::JOY[77],
    // („• ᴗ •„)
    kaomoji_ru::positive_emotions::JOY[84],
    // (´ ω `♡)
    kaomoji_ru::positive_emotions::LOVE[12],
    // (♡°▽°♡)
    kaomoji_ru::positive_emotions::LOVE[17],
    // ♡(｡- ω -)
    kaomoji_ru::positive_emotions::LOVE[18],
    // (´｡• ω •｡`) ♡
    kaomoji_ru::positive_emotions::LOVE[22],
    // (❤ω❤)
    kaomoji_ru::positive_emotions::LOVE[39],
    // (´,,•ω•,,)♡
    kaomoji_ru::positive_emotions::LOVE[45],
    // (*ﾉωﾉ)
    kaomoji_ru::positive_emotions::EMBARRESMENT[5],
    // (⁄ ⁄•⁄ω⁄•⁄ ⁄)
    kaomoji_ru::positive_emotions::EMBARRESMENT[17],
    // (＃￣ω￣)
    kaomoji_ru::negative_emotions::DISSATISFACTION[7],
    // (＞ｍ＜)
    kaomoji_ru::negative_emotions::DISSATISFACTION[9],
    // (」°ロ°)」
    kaomoji_ru::negative_emotions::DISSATISFACTION[10],
    // (ᗒᗣᗕ)՞
    kaomoji_ru::negative_emotions::DISSATISFACTION[24],
    // (＃`Д´)
    kaomoji_ru::negative_emotions::ANGER[0],
    // (・`ω´・)
    kaomoji_ru::negative_emotions::ANGER[4],
    // (°ㅂ°╬)
    kaomoji_ru::negative_emotions::ANGER[17],
    // (╬ Ò﹏Ó)
    kaomoji_ru::negative_emotions::ANGER[25],
    // (´-ω-`)
    kaomoji_ru::negative_emotions::SADNESS[2],
    // (-ω-、)
    kaomoji_ru::negative_emotions::SADNESS[6],
    // ( ; ω ; )
    kaomoji_ru::negative_emotions::SADNESS[9],
    // ( ╥ω╥ )
    kaomoji_ru::negative_emotions::SADNESS[16],
    // (ノωヽ)
    kaomoji_ru::negative_emotions::FEAR[0],
    // (・_・ヾ
    kaomoji_ru::neutral_emotions::CONFUSSION[5],
    // ╮(￣ω￣;)╭
    kaomoji_ru::neutral_emotions::CONFUSSION[10],
    // (*・ω・)ﾉ
    kaomoji_ru::various_actions::GREETING[0],
    // (✧∀✧)/
    kaomoji_ru::various_actions::GREETING[25],
    // (つ≧▽≦)つ
    kaomoji_ru::various_actions::HUGGING[1],
    // (つ✧ω✧)つ
    kaomoji_ru::various_actions::HUGGING[2],
    // ⊂(´• ω •`⊂)
    kaomoji_ru::various_actions::HUGGING[8],
    // ⊂(･ω･*⊂)
    kaomoji_ru::various_actions::HUGGING[9],
    // (^ω~)
    kaomoji_ru::various_actions::WINKING[3],
    // |･ω･)
    kaomoji_ru::various_actions::HIDING[0],
    // ☆ﾐ(o*･ω･)ﾉ
    kaomoji_ru::various_actions::RUNNING[0],
    // C= C= C= C= C=┌(;・ω・)┘
    kaomoji_ru::various_actions::RUNNING[1],
    // ε===(っ≧ω≦)っ
    kaomoji_ru::various_actions::RUNNING[6],
    // (－ω－) zzZ
    kaomoji_ru::various_actions::SLEEPING[3],
    // (=^･ω･^=)
    kaomoji_ru::animals::CAT[0],
    // (=^･ｪ･^=)
    kaomoji_ru::animals::CAT[1],
    // (=①ω①=)
    kaomoji_ru::animals::CAT[2],
    // ( =ω=)..nyaa
    kaomoji_ru::animals::CAT[3],
    // (= ; ｪ ; =)
    kaomoji_ru::animals::CAT[4],
    // (=`ω´=)
    kaomoji_ru::animals::CAT[5],
    // (=^‥^=)
    kaomoji_ru::animals::CAT[6],
    // ( =ノωヽ=)
    kaomoji_ru::animals::CAT[9],
    // (=^ ◡ ^=)
    kaomoji_ru::animals::CAT[11],
    // (=^-ω-^=)
    kaomoji_ru::animals::CAT[12],
    // ヾ(=`ω´=)ノ”
    kaomoji_ru::animals::CAT[13],
    // (＾• ω •＾)
    kaomoji_ru::animals::CAT[14],
    // (/ =ω=)/
    kaomoji_ru::animals::CAT[15],
    // ฅ(•ㅅ•❀)ฅ
    kaomoji_ru::animals::CAT[16],
    // ଲ(ⓛ ω ⓛ)ଲ
    kaomoji_ru::animals::CAT[18],
    // (^=◕ᴥ◕=^)
    kaomoji_ru::animals::CAT[19],
    // ( =ω= )
    kaomoji_ru::animals::CAT[20],
    // (^◔ᴥ◔^)
    kaomoji_ru::animals::CAT[25],
    // (　･ω･)☞
    kaomoji_ru::special::POINTING,
];

pub const ACTIONS_SIZE: usize = 17;
pub const ACTIONS: [&[u8]; ACTIONS_SIZE] = [
    b"*notices bulge* ",
    b"*cries* ",
    b"*hugs tightly* ",
    b"*screams* ",
    b"*looks away* ",
    b"*blushes* ",
    b"*sweats* ",
    b"*cuddles you* ",
    b"*moans* ",
    b"*giggles shyly* ",
    b"*looks at you* ",
    b"*twerks* ",
    b"*sighs* ",
    b"*leans over* ",
    b"*pokes you* ",
    b"*teleports behind you* ",
    b"*shuffles closer* ",
];
