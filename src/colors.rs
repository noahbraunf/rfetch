use ansi_term::Color::*;
pub fn get_16_terminal_colors() -> String {
    format!(
        "{c1}{c2}{c3}{c4}{c5}{c6}{c7}{c8}\n{c9}{c10}{c11}{c12}{c13}{c14}{c15}{c16}",
        c1 = Black.reverse().paint("   "),
        c2 = Red.reverse().paint("   "),
        c3 = Green.reverse().paint("   "),
        c4 = Yellow.reverse().paint("   "),
        c5 = Blue.reverse().paint("   "),
        c6 = Purple.reverse().paint("   "),
        c7 = Cyan.reverse().paint("   "),
        c8 = White.reverse().paint("   "),
        c9 = Black.bold().reverse().reverse().paint("   "),
        c10 = Red.bold().reverse().paint("   "),
        c11 = Green.bold().reverse().paint("   "),
        c12 = Yellow.bold().reverse().paint("   "),
        c13 = Blue.bold().reverse().paint("   "),
        c14 = Purple.bold().reverse().paint("   "),
        c15 = Cyan.bold().reverse().paint("   "),
        c16 = White.bold().reverse().paint("   ")
    )
}
