use crate::cli::AsciiChar;

pub static DEFAULT_FONT_DATA: &[AsciiChar] = &[AsciiChar { character: 'A', ascii: ["           ", "     /\\    ", "    /  \\   ", "   / /\\ \\  ", "  / ____ \\ ", " /_/    \\_\\", "           ", "           "] }, AsciiChar { character: 'B', ascii: ["  ____  ", " |  _ \\ ", " | |_) |", " |  _ < ", " | |_) |", " |____/ ", "        ", "        "] }, AsciiChar { character: 'C', ascii: ["   _____ ", "  / ____|", " | |     ", " | |     ", " | |____ ", "  \\_____|", "         ", "         "] }, AsciiChar { character: 'D', ascii: ["  _____  ", " |  __ \\ ", " | |  | |", " | |  | |", " | |__| |", " |_____/ ", "         ", "         "] }, AsciiChar { character: 'E', ascii: ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |____ ", " |______|", "         ", "         "] }, AsciiChar { character: 'F', ascii: ["  ______ ", " |  ____|", " | |__   ", " |  __|  ", " | |     ", " |_|     ", "         ", "         "] }, AsciiChar { character: 'G', ascii: ["   _____ ", "  / ____|", " | |  __ ", " | | |_ |", " | |__| |", "  \\_____|", "         ", "         "] }, AsciiChar { character: 'H', ascii: ["  _    _ ", " | |  | |", " | |__| |", " |  __  |", " | |  | |", " |_|  |_|", "         ", "         "] }, AsciiChar { character: 'I', ascii: ["  _____ ", " |_   _|", "   | |  ", "   | |  ", "  _| |_ ", " |_____|", "        ", "        "] }, AsciiChar { character: 'J', ascii: ["       _ ", "      | |", "      | |", "  _   | |", " | |__| |", "  \\____/ ", "         ", "         "] }, AsciiChar { character: 'K', ascii: ["  _  __", " | |/ /", " | ' / ", " |  <  ", " | . \\ ", " |_|\\_\\", "       ", "       "] }, AsciiChar { character: 'L', ascii: ["  _      ", " | |     ", " | |     ", " | |     ", " | |____ ", " |______|", "         ", "         "] }, AsciiChar { character: 'M', ascii: ["  __  __ ", " |  \\/  |", " | \\  / |", " | |\\/| |", " | |  | |", " |_|  |_|", "         ", "         "] }, AsciiChar { character: 'N', ascii: ["  _   _ ", " | \\ | |", " |  \\| |", " | . ` |", " | |\\  |", " |_| \\_|", "        ", "        "] }, AsciiChar { character: 'O', ascii: ["   ____  ", "  / __ \\ ", " | |  | |", " | |  | |", " | |__| |", "  \\____/ ", "         ", "         "] }, AsciiChar { character: 'P', ascii: ["  _____  ", " |  __ \\ ", " | |__) |", " |  ___/ ", " | |     ", " |_|     ", "         ", "         "] }, AsciiChar { character: 'Q', ascii: ["   ____  ", "  / __ \\ ", " | |  | |", " | |  | |", " | |__| |", "  \\___\\_\\", "         ", "         "] }, AsciiChar { character: 'R', ascii: ["  _____  ", " |  __ \\ ", " | |__) |", " |  _  / ", " | | \\ \\ ", " |_|  \\_\\", "         ", "         "] }, AsciiChar { character: 'S', ascii: ["   _____ ", "  / ____|", " | (___  ", "  \\___ \\ ", "  ____) |", " |_____/ ", "         ", "         "] }, AsciiChar { character: 'T', ascii: ["  _______ ", " |__   __|", "    | |   ", "    | |   ", "    | |   ", "    |_|   ", "          ", "          "] }, AsciiChar { character: 'U', ascii: ["  _    _ ", " | |  | |", " | |  | |", " | |  | |", " | |__| |", "  \\____/ ", "         ", "         "] }, AsciiChar { character: 'V', ascii: [" __      __", " \\ \\    / /", "  \\ \\  / / ", "   \\ \\/ /  ", "    \\  /   ", "     \\/    ", "           ", "           "] }, AsciiChar { character: 'W', ascii: [" __          __", " \\ \\        / /", "  \\ \\  /\\  / / ", "   \\ \\/  \\/ /  ", "    \\  /\\  /   ", "     \\/  \\/    ", "               ", "               "] }, AsciiChar { character: 'X', ascii: [" __   __", " \\ \\ / /", "  \\ V / ", "   > <  ", "  / . \\ ", " /_/ \\_\\", "        ", "        "] }, AsciiChar { character: 'Y', ascii: [" __     __", " \\ \\   / /", "  \\ \\_/ / ", "   \\   /  ", "    | |   ", "    |_|   ", "          ", "          "] }, AsciiChar { character: 'Z', ascii: ["  ______", " |___  /", "    / / ", "   / /  ", "  / /__ ", " /_____|", "        ", "        "] }, AsciiChar { character: 'a', ascii: ["        ", "        ", "   __ _ ", "  / _` |", " | (_| |", "  \\__,_|", "        ", "        "] }, AsciiChar { character: 'b', ascii: ["  _     ", " | |    ", " | |__  ", " | '_ \\ ", " | |_) |", " |_.__/ ", "        ", "        "] }, AsciiChar { character: 'c', ascii: ["       ", "       ", "   ___ ", "  / __|", " | (__ ", "  \\___|", "       ", "       "] }, AsciiChar { character: 'd', ascii: ["      _ ", "     | |", "   __| |", "  / _` |", " | (_| |", "  \\__,_|", "        ", "        "] }, AsciiChar { character: 'e', ascii: ["       ", "       ", "   ___ ", "  / _ \\", " |  __/", "  \\___|", "       ", "       "] }, AsciiChar { character: 'f', ascii: ["   __ ", "  / _|", " | |_ ", " |  _|", " | |  ", " |_|  ", "      ", "      "] }, AsciiChar { character: 'g', ascii: ["        ", "        ", "   __ _ ", "  / _` |", " | (_| |", "  \\__, |", "   __/ |", "  |___/ "] }, AsciiChar { character: 'h', ascii: ["  _     ", " | |    ", " | |__  ", " | '_ \\ ", " | | | |", " |_| |_|", "        ", "        "] }, AsciiChar { character: 'i', ascii: ["  _ ", " (_)", "  _ ", " | |", " | |", " |_|", "    ", "    "] }, AsciiChar { character: 'j', ascii: ["    _ ", "   (_)", "    _ ", "   | |", "   | |", "   | |", "  _/ |", " |__/ "] }, AsciiChar { character: 'k', ascii: ["  _    ", " | |   ", " | | __", " | |/ /", " |   < ", " |_|\\_\\", "       ", "       "] }, AsciiChar { character: 'l', ascii: ["  _ ", " | |", " | |", " | |", " | |", " |_|", "    ", "    "] }, AsciiChar { character: 'm', ascii: ["            ", "            ", "  _ __ ___  ", " | '_ ` _ \\ ", " | | | | | |", " |_| |_| |_|", "            ", "            "] }, AsciiChar { character: 'n', ascii: ["        ", "        ", "  _ __  ", " | '_ \\ ", " | | | |", " |_| |_|", "        ", "        "] }, AsciiChar { character: 'o', ascii: ["        ", "        ", "   ___  ", "  / _ \\ ", " | (_) |", "  \\___/ ", "        ", "        "] }, AsciiChar { character: 'p', ascii: ["        ", "        ", "  _ __  ", " | '_ \\ ", " | |_) |", " | .__/ ", " | |    ", " |_|    "] }, AsciiChar { character: 'q', ascii: ["        ", "        ", "   __ _ ", "  / _` |", " | (_| |", "  \\__, |", "     | |", "     |_|"] }, AsciiChar { character: 'r', ascii: ["       ", "       ", "  _ __ ", " | '__|", " | |   ", " |_|   ", "       ", "       "] }, AsciiChar { character: 's', ascii: ["      ", "      ", "  ___ ", " / __|", " \\__ \\", " |___/", "      ", "      "] }, AsciiChar { character: 't', ascii: ["  _   ", " | |  ", " | |_ ", " | __|", " | |_ ", "  \\__|", "      ", "      "] }, AsciiChar { character: 'u', ascii: ["        ", "        ", "  _   _ ", " | | | |", " | |_| |", "  \\__,_|", "        ", "        "] }, AsciiChar { character: 'v', ascii: ["        ", "        ", " __   __", " \\ \\ / /", "  \\ V / ", "   \\_/  ", "        ", "        "] }, AsciiChar { character: 'w', ascii: ["           ", "           ", " __      __", " \\ \\ /\\ / /", "  \\ V  V / ", "   \\_/\\_/  ", "           ", "           "] }, AsciiChar { character: 'x', ascii: ["       ", "       ", " __  __", " \\ \\/ /", "  >  < ", " /_/\\_\\", "       ", "       "] }, AsciiChar { character: 'y', ascii: ["        ", "        ", "  _   _ ", " | | | |", " | |_| |", "  \\__, |", "   __/ |", "  |___/ "] }, AsciiChar { character: 'z', ascii: ["      ", "      ", "  ____", " |_  /", "  / / ", " /___|", "      ", "      "] }, AsciiChar { character: '~', ascii: ["  /\\/|", " |/\\/ ", "      ", "      ", "      ", "      ", "      ", "      "] }, AsciiChar { character: '!', ascii: ["  _ ", " | |", " | |", " | |", " |_|", " (_)", "    ", "    "] }, AsciiChar { character: '@', ascii: ["          ", "    ____  ", "   / __ \\ ", "  / / _` |", " | | (_| |", "  \\ \\__,_|", "   \\____/ ", "          "] }, AsciiChar { character: '#', ascii: ["    _  _   ", "  _| || |_ ", " |_  __  _|", "  _| || |_ ", " |_  __  _|", "   |_||_|  ", "           ", "           "] }, AsciiChar { character: '$', ascii: ["   _  ", "  | | ", " / __)", " \\__ \\", " (   /", "  |_| ", "      ", "      "] }, AsciiChar { character: '%', ascii: ["  _   __", " (_) / /", "    / / ", "   / /  ", "  / / _ ", " /_/ (_)", "        ", "        "] }, AsciiChar { character: '^', ascii: ["  /\\ ", " |/\\|", "     ", "     ", "     ", "     ", "     ", "     "] }, AsciiChar { character: '&', ascii: ["         ", "   ___   ", "  ( _ )  ", "  / _ \\/\\", " | (_>  <", "  \\___/\\/", "         ", "         "] }, AsciiChar { character: '*', ascii: ["     _    ", "  /\\| |/\\ ", "  \\ ` ' / ", " |_     _|", "  / , . \\ ", "  \\/|_|\\/ ", "          ", "          "] }, AsciiChar { character: '(', ascii: ["   __", "  / /", " | | ", " | | ", " | | ", " | | ", "  \\_\\", "     "] }, AsciiChar { character: ')', ascii: [" __  ", " \\ \\ ", "  | |", "  | |", "  | |", "  | |", " /_/ ", "     "] }, AsciiChar { character: '_', ascii: ["         ", "         ", "         ", "         ", "         ", "         ", "  ______ ", " |______|"] }, AsciiChar { character: '+', ascii: ["        ", "    _   ", "  _| |_ ", " |_   _|", "   |_|  ", "        ", "        ", "        "] }, AsciiChar { character: '|', ascii: ["  _ ", " | |", " | |", " | |", " | |", " | |", " | |", " |_|"] }, AsciiChar { character: '-', ascii: ["         ", "         ", "  ______ ", " |______|", "         ", "         ", "         ", "         "] }, AsciiChar { character: '=', ascii: ["         ", "  ______ ", " |______|", "  ______ ", " |______|", "         ", "         ", "         "] }, AsciiChar { character: '{', ascii: ["    __", "   / /", "  | | ", " / /  ", " \\ \\  ", "  | | ", "   \\_\\", "      "] }, AsciiChar { character: '}', ascii: [" __   ", " \\ \\  ", "  | | ", "   \\ \\", "   / /", "  | | ", " /_/  ", "      "] }, AsciiChar { character: '[', ascii: ["  ___ ", " |  _|", " | |  ", " | |  ", " | |  ", " | |_ ", " |___|", "      "] }, AsciiChar { character: ']', ascii: ["  ___ ", " |_  |", "   | |", "   | |", "   | |", "  _| |", " |___|", "      "] }, AsciiChar { character: ':', ascii: ["    ", "  _ ", " (_)", "    ", "  _ ", " (_)", "    ", "    "] }, AsciiChar { character: ';', ascii: ["    ", "  _ ", " (_)", "    ", "  _ ", " ( )", " |/ ", "    "] }, AsciiChar { character: '\'', ascii: ["  _ ", " ( )", " |/ ", "    ", "    ", "    ", "    ", "    "] }, AsciiChar { character: '"', ascii: ["  _ _ ", " ( | )", "  V V ", "      ", "      ", "      ", "      ", "      "] }, AsciiChar { character: ',', ascii: ["    ", "    ", "    ", "    ", "  _ ", " ( )", " |/ ", "    "] }, AsciiChar { character: '.', ascii: ["    ", "    ", "    ", "    ", "  _ ", " (_)", "    ", "    "] }, AsciiChar { character: '<', ascii: ["    __", "   / /", "  / / ", " < <  ", "  \\ \\ ", "   \\_\\", "      ", "      "] }, AsciiChar { character: '>', ascii: [" __   ", " \\ \\  ", "  \\ \\ ", "   > >", "  / / ", " /_/  ", "      ", "      "] }, AsciiChar { character: '?', ascii: ["  ___  ", " |__ \\ ", "    ) |", "   / / ", "  |_|  ", "  (_)  ", "       ", "       "] }, AsciiChar { character: '/', ascii: ["      __", "     / /", "    / / ", "   / /  ", "  / /   ", " /_/    ", "        ", "        "] }, AsciiChar { character: '\\', ascii: [" __     ", " \\ \\    ", "  \\ \\   ", "   \\ \\  ", "    \\ \\ ", "     \\_\\", "        ", "        "] }, AsciiChar { character: '`', ascii: ["  _ ", " ( )", "  \\|", "    ", "    ", "    ", "    ", "    "] }, AsciiChar { character: '1', ascii: ["  __ ", " /_ |", "  | |", "  | |", "  | |", "  |_|", "     ", "     "] }, AsciiChar { character: '2', ascii: ["  ___  ", " |__ \\ ", "    ) |", "   / / ", "  / /_ ", " |____|", "       ", "       "] }, AsciiChar { character: '3', ascii: ["  ____  ", " |___ \\ ", "   __) |", "  |__ < ", "  ___) |", " |____/ ", "        ", "        "] }, AsciiChar { character: '4', ascii: ["  _  _   ", " | || |  ", " | || |_ ", " |__   _|", "    | |  ", "    |_|  ", "         ", "         "] }, AsciiChar { character: '5', ascii: ["  _____ ", " | ____|", " | |__  ", " |___ \\ ", "  ___) |", " |____/ ", "        ", "        "] }, AsciiChar { character: '6', ascii: ["    __  ", "   / /  ", "  / /_  ", " | '_ \\ ", " | (_) |", "  \\___/ ", "        ", "        "] }, AsciiChar { character: '7', ascii: ["  ______ ", " |____  |", "     / / ", "    / /  ", "   / /   ", "  /_/    ", "         ", "         "] }, AsciiChar { character: '8', ascii: ["   ___  ", "  / _ \\ ", " | (_) |", "  > _ < ", " | (_) |", "  \\___/ ", "        ", "        "] }, AsciiChar { character: '9', ascii: ["   ___  ", "  / _ \\ ", " | (_) |", "  \\__, |", "    / / ", "   /_/  ", "        ", "        "] }, AsciiChar { character: '0', ascii: ["   ___  ", "  / _ \\ ", " | | | |", " | | | |", " | |_| |", "  \\___/ ", "        ", "        "] }, AsciiChar { character: ' ', ascii: ["      ", "      ", "      ", "      ", "      ", "      ", "      ", "      "] }];