pub const ASCII_FACES_SIZE: usize = 33;
pub const ASCII_FACES: [&[u8]; ASCII_FACES_SIZE] = [
    b"OwO ",
    b"UwU ",
    b">w< ",
    b"^w^ ",
    b"^-^ ",
    b":3 ",
    b"x3 ",
    b"xDD ",
    b";;w;; ",
    b">_< ",
    b">_> ",
    b"^.^ ",
    b":33 ",
    b"uWu ",
    b"(o^ ^o) ",
    b"(o-_-o) ",
    b"(*^.^*) ",
    b"(--_--) ",
    b"o(>< )o ",
    b"(-_-) ",
    b"(T_T) ",
    b"(>_<) ",
    b"~(>_<~) ",
    b"(x_x)V ",
    b"(;;;*_*) ",
    b"{{ (>_<) }} ",
    b"(o_O) ",
    b"(O_O;) ",
    b"(O.O) ",
    b"(o_O)! ",
    b"(^-^*)/ ",
    b"(o^ ^o)/ ",
    b"( ~*-*)~ ",
];

pub const UNICODE_FACES_SIZE: usize = 66;
pub const UNICODE_FACES: [&[u8]; UNICODE_FACES_SIZE] = [
    // (* ^ ω ^)
    b"(* ^ \xCF\x89 ^) ",
    // (´ ∀ ` *)
    b"(\xC2\xB4 \xE2\x88\x80 ` *) ",
    // (o^▽^o)
    b"(o^\xE2\x96\xBD^o) ",
    // (⌒▽⌒)☆
    b"(\xE2\x8C\x92\xE2\x96\xBD\xE2\x8C\x92)\xE2\x98\x86 ",
    // ヽ(・∀・)ﾉ
    b"\xE3\x83\xBD(\xE3\x83\xBB\xE2\x88\x80\xE3\x83\xBB)\xEF\xBE\x89 ",
    // (￣ω￣)
    b"(\xEF\xBF\xA3\xCF\x89\xEF\xBF\xA3) ",
    // (o･ω･o)
    b"(o\xEF\xBD\xA5\xCF\x89\xEF\xBD\xA5o) ",
    // (^人^)
    b"(^\xE4\xBA\xBA^) ",
    // (*´▽`*)
    b"(*\xC2\xB4\xE2\x96\xBD`*) ",
    // (≧◡≦)
    b"(\xE2\x89\xA7\xE2\x97\xA1\xE2\x89\xA6) ",
    // (o´∀`o)
    b"(o\xC2\xB4\xE2\x88\x80`o) ",
    // (＾▽＾)
    b"(\xEF\xBC\xBE\xE2\x96\xBD\xEF\xBC\xBE') ",
    // (⌒ω⌒)
    b"(\xE2\x8C\x92\xCF\x89\xE2\x8C\x92) ",
    // ╰(▔∀▔)╯
    b"\xE2\x95\xB0(\xE2\x96\x94\xE2\x88\x80\xE2\x96\x94)\xE2\x95\xAF ",
    // (*^‿^*)
    b"(*^\xE2\x80\xBF^*) ",
    // (✯◡✯)
    b"(\xE2\x9C\xAF\xE2\x97\xA1\xE2\x9C\xAF) ",
    // (*≧ω≦*)
    b"(*\xE2\x89\xA7\xCF\x89\xE2\x89\xA6*) ",
    // (☆▽☆)
    b"(\xE2\x98\x86\xE2\x96\xBD\xE2\x98\x86) ",
    // ＼(≧▽≦)／
    b"\xEF\xBC\xBC(\xE2\x89\xA7\xE2\x96\xBD\xE2\x89\xA6)\xEF\xBC\x8F ",
    // (*°▽°*)
    b"(*\xC2\xB0\xE2\x96\xBD\xC2\xB0*) ",
    // (✧ω✧)
    b"(\xE2\x9C\xA7\xCF\x89\xE2\x9C\xA7) ",
    // ヽ(>∀<☆)ノ
    b"\xE3\x83\xBD(>\xE2\x88\x80<\xE2\x98\x86)\xE3\x83\x8E ",
    // o(≧▽≦)o
    b"o(\xE2\x89\xA7\xE2\x96\xBD\xE2\x89\xA6)o ",
    // (☆ω☆)
    b"(\xE2\x98\x86\xCF\x89\xE2\x98\x86) ",
    // (っ˘ω˘ς )
    b"(\xE3\x81\xA3\xCB\x98\xCF\x89\xCB\x98\xCF\x82 ) ",
    // \(★ω★)/
    b"\\(\xE2\x98\x85\xCF\x89\xE2\x98\x85)/ ",
    // (╯✧▽✧)╯
    b"(\xE2\x95\xAF\xE2\x9C\xA7\xE2\x96\xBD\xE2\x9C\xA7)\xE2\x95\xAF ",
    // o(>ω<)o
    b"o(>\xCF\x89<)o ",
    // (´ ω `♡)
    b"(\xC2\xB4 \xCF\x89 `\xE2\x99\xA1) ",
    // (♡°▽°♡)
    b"(\xE2\x99\xA1\xC2\xB0\xE2\x96\xBD\xC2\xB0\xE2\x99\xA1) ",
    // ♡(｡- ω -)
    b"\xE2\x99\xA1(\xEF\xBD\xA1- \xCF\x89 -) ",
    // (❤ω❤)
    b"(\xE2\x9D\xA4\xCF\x89\xE2\x9D\xA4) ",
    // (*ﾉωﾉ)
    b"(*\xEF\xBE\x89\xCF\x89\xEF\xBE\x89) ",
    // (＃￣ω￣)
    b"(\xEF\xBC\x83\xEF\xBF\xA3\xCF\x89\xEF\xBF\xA3) ",
    // (＞ｍ＜)
    b"(\xEF\xBC\x9E\xEF\xBD\x8D\xEF\xBC\x9C) ",
    // (ᗒᗣᗕ)՞
    b"(\xE1\x97\x92\xE1\x97\xA3\xE1\x97\x95)\xD5\x9E ",
    // (＃`Д´)
    b"(\xEF\xBC\x83`\xD0\x94\xC2\xB4) ",
    // (°ㅂ°╬)
    b"(\xC2\xB0\xE3\x85\x82\xC2\xB0\xE2\x95\xAC) ",
    // (╬ Ò﹏Ó)
    b"(\xE2\x95\xAC \xC3\x92\xEF\xB9\x8F\xC3\x93) ",
    // (´-ω-`)
    b"(\xC2\xB4-\xCF\x89-`) ",
    // (-ω-、)
    b"(-\xCF\x89-\xE3\x80\x81) ",
    // ( ╥ω╥ )
    b"( \xE2\x95\xA5\xCF\x89\xE2\x95\xA5 ) ",
    // (ノωヽ)
    b"(\xE3\x83\x8E\xCF\x89\xE3\x83\xBD) ",
    // (・_・ヾ
    b"(\xE3\x83\xBB_\xE3\x83\xBB\xE3\x83\xBE ",
    // ╮(￣ω￣;)╭
    b"\xE2\x95\xAE(\xEF\xBF\xA3\xCF\x89\xEF\xBF\xA3;)\xE2\x95\xAD ",
    // (*・ω・)ﾉ
    b"(*\xE3\x83\xBB\xCF\x89\xE3\x83\xBB)\xEF\xBE\x89 ",
    // (✧∀✧)/
    b"(\xE2\x9C\xA7\xE2\x88\x80\xE2\x9C\xA7)/ ",
    // (つ≧▽≦)つ
    b"(\xE3\x81\xA4\xE2\x89\xA7\xE2\x96\xBD\xE2\x89\xA6)\xE3\x81\xA4 ",
    // (つ✧ω✧)つ
    b"(\xE3\x81\xA4\xE2\x9C\xA7\xCF\x89\xE2\x9C\xA7)\xE3\x81\xA4 ",
    // ⊂(･ω･*⊂)
    b"\xE2\x8A\x82(\xEF\xBD\xA5\xCF\x89\xEF\xBD\xA5*\xE2\x8A\x82) ",
    // (^ω~)
    b"(^\xCF\x89~) ",
    // |･ω･)
    b"|\xEF\xBD\xA5\xCF\x89\xEF\xBD\xA5) ",
    // (=^･ω･^=)
    b"(=^\xEF\xBD\xA5\xCF\x89\xEF\xBD\xA5^=) ",
    // (=^･ｪ･^=)
    b"(=^\xEF\xBD\xA5\xEF\xBD\xAA\xEF\xBD\xA5^=) ",
    // (=①ω①=)
    b"(=\xE2\x91\xA0\xCF\x89\xE2\x91\xA0=) ",
    // ( =ω=)..nyaa
    b"( =\xCF\x89=)..nyaa ",
    // (=`ω´=)
    b"(=`\xCF\x89\xC2\xB4=) ",
    // (=^‥^=)
    b"(=^\xE2\x80\xA5^=) ",
    // ( =ノωヽ=)
    b"(=^ \xE2\x97\xA1 ^=) ",
    // (=^-ω-^=)
    b"(\xEF\xBC\xBE\xE2\x80\xA2 \xCF\x89 \xE2\x80\xA2\xEF\xBC\xBE) ",
    // ヾ(=`ω´=)ノ”
    b"(/ =\xCF\x89=)/ ",
    // (/ =ω=)/
    b"\xE0\xB8\x85(\xE2\x80\xA2 \xC9\xAA \xE2\x80\xA2)\xE0\xB8\x85 ",
    // ฅ(•ㅅ•❀)ฅ
    b"\xE0\xAC\xB2(\xE2\x93\x9B \xCF\x89 \xE2\x93\x9B)\xE0\xAC\xB2 ",
    // ( =ω= )
    b"(^=\xE2\x97\x95\xE1\xB4\xA5\xE2\x97\x95=^) ",
    // (^◔ᴥ◔^)
    b"\xE0\xB8\x95(=\xCF\x89=)\xE0\xB8\x95 ",
    // (　･ω･)☞
    b"(\xE3\x80\x80\xEF\xBD\xA5\xCF\x89\xEF\xBD\xA5)\xE2\x98\x9E ",
];

pub const MIXED_FACES_SIZE: usize = mixed_len(&ASCII_FACES, &UNICODE_FACES);
pub const MIXED_FACES: [&[u8]; MIXED_FACES_SIZE] = mixed_array(&ASCII_FACES, &UNICODE_FACES);

pub const fn mixed_len(ascii_array: &[&[u8]], unicode_array: &[&[u8]]) -> usize {
    unicode_array.len() + ascii_array.len()
}

pub const fn mixed_array(
    ascii_array: &'static [&'static [u8]],
    unicode_array: &'static [&'static [u8]],
) -> [&'static [u8]; MIXED_FACES_SIZE] {
    let mut mixed_array: [&'static [u8]; MIXED_FACES_SIZE] = [&[]; MIXED_FACES_SIZE];
    let ascii_len = ascii_array.len();
    let mut count = 0;
    let mut second_count = 0;

    while count < ascii_len {
        mixed_array[count] = ascii_array[count];
        count += 1;
    }

    while count < MIXED_FACES_SIZE {
        mixed_array[count] = unicode_array[second_count];
        count += 1;
        second_count += 1;
    }

    mixed_array
}

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