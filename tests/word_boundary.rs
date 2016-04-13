// Many of these are cribbed from RE2's test suite.

matiter!(wb1, r"\b", "");
matiter!(wb2, r"\b", "a", (0, 0), (1, 1));
matiter!(wb3, r"\b", "ab", (0, 0), (2, 2));
matiter!(wb4, r"^\b", "ab", (0, 0));
matiter!(wb5, r"\b$", "ab", (2, 2));
matiter!(wb6, r"^\b$", "ab");
matiter!(wb7, r"\bbar\b", "nobar bar foo bar", (6, 9), (14, 17));
matiter!(wb8, r"a\b", "faoa x", (3, 4));
matiter!(wb9, r"\bbar", "bar x", (0, 3));
matiter!(wb10, r"\bbar", "foo\nbar x", (4, 7));
matiter!(wb11, r"bar\b", "foobar", (3, 6));
matiter!(wb12, r"bar\b", "foobar\nxxx", (3, 6));
matiter!(wb13, r"(foo|bar|[A-Z])\b", "foo", (0, 3));
matiter!(wb14, r"(foo|bar|[A-Z])\b", "foo\n", (0, 3));
matiter!(wb15, r"\b(foo|bar|[A-Z])", "foo", (0, 3));
matiter!(wb16, r"\b(foo|bar|[A-Z])\b", "X", (0, 1));
matiter!(wb17, r"\b(foo|bar|[A-Z])\b", "XY");
matiter!(wb18, r"\b(foo|bar|[A-Z])\b", "bar", (0, 3));
matiter!(wb19, r"\b(foo|bar|[A-Z])\b", "foo", (0, 3));
matiter!(wb20, r"\b(foo|bar|[A-Z])\b", "foo\n", (0, 3));
matiter!(wb21, r"\b(foo|bar|[A-Z])\b", "ffoo bbar N x", (10, 11));
matiter!(wb22, r"\b(fo|foo)\b", "fo", (0, 2));
matiter!(wb23, r"\b(fo|foo)\b", "foo", (0, 3));
matiter!(wb24, r"\b\b", "");
matiter!(wb25, r"\b\b", "a", (0, 0), (1, 1));
matiter!(wb26, r"\b$", "");
matiter!(wb27, r"\b$", "x", (1, 1));
matiter!(wb28, r"\b$", "y x", (3, 3));
matiter!(wb29, r"\b.$", "x", (0, 1));
matiter!(wb30, r"^\b(fo|foo)\b", "fo", (0, 2));
matiter!(wb31, r"^\b(fo|foo)\b", "foo", (0, 3));
matiter!(wb32, r"^\b$", "");
matiter!(wb33, r"^\b$", "x");
matiter!(wb34, r"^\b.$", "x", (0, 1));
matiter!(wb35, r"^\b.\b$", "x", (0, 1));
matiter!(wb36, r"^^^^^\b$$$$$", "");
matiter!(wb37, r"^^^^^\b.$$$$$", "x", (0, 1));
matiter!(wb38, r"^^^^^\b$$$$$", "x");
matiter!(wb39, r"^^^^^\b\b\b.\b\b\b$$$$$", "x", (0, 1));
matiter!(wb40, r"\b.+\b", "$$abc$$", (2, 5));

matiter!(nb1, r"\Bfoo\B", "n foo xfoox that", (7, 10));
matiter!(nb2, r"a\B", "faoa x", (1, 2));
matiter!(nb3, r"\Bbar", "bar x");
matiter!(nb4, r"\Bbar", "foo\nbar x");
matiter!(nb5, r"bar\B", "foobar");
matiter!(nb6, r"bar\B", "foobar\nxxx");
matiter!(nb7, r"(foo|bar|[A-Z])\B", "foox", (0, 3));
matiter!(nb8, r"(foo|bar|[A-Z])\B", "foo\n");
matiter!(nb9, r"\B", "", (0, 0));
matiter!(nb10, r"\B", "x");
matiter!(nb11, r"\B(foo|bar|[A-Z])", "foo");
matiter!(nb12, r"\B(foo|bar|[A-Z])\B", "xXy", (1, 2));
matiter!(nb13, r"\B(foo|bar|[A-Z])\B", "XY");
matiter!(nb14, r"\B(foo|bar|[A-Z])\B", "XYZ", (1, 2));
matiter!(nb15, r"\B(foo|bar|[A-Z])\B", "abara", (1, 4));
matiter!(nb16, r"\B(foo|bar|[A-Z])\B", "xfoo_", (1, 4));
matiter!(nb17, r"\B(foo|bar|[A-Z])\B", "xfoo\n");
matiter!(nb18, r"\B(foo|bar|[A-Z])\B", "foo bar vNX", (9, 10));
matiter!(nb19, r"\B(fo|foo)\B", "xfoo", (1, 3));
matiter!(nb20, r"\B(foo|fo)\B", "xfooo", (1, 4));
matiter!(nb21, r"\B\B", "", (0, 0));
matiter!(nb22, r"\B\B", "x");
matiter!(nb23, r"\B$", "", (0, 0));
matiter!(nb24, r"\B$", "x");
matiter!(nb25, r"\B$", "y x");
matiter!(nb26, r"\B.$", "x");
matiter!(nb27, r"^\B(fo|foo)\B", "fo");
matiter!(nb28, r"^\B(fo|foo)\B", "foo");
matiter!(nb29, r"^\B", "", (0, 0));
matiter!(nb30, r"^\B", "x");
matiter!(nb31, r"^\B\B", "", (0, 0));
matiter!(nb32, r"^\B\B", "x");
matiter!(nb33, r"^\B$", "", (0, 0));
matiter!(nb34, r"^\B$", "x");
matiter!(nb35, r"^\B.$", "x");
matiter!(nb36, r"^\B.\B$", "x");
matiter!(nb37, r"^^^^^\B$$$$$", "", (0, 0));
matiter!(nb38, r"^^^^^\B.$$$$$", "x");
matiter!(nb39, r"^^^^^\B$$$$$", "x");

// These work for both Unicode and ASCII because all matches are reported as
// byte offsets, and « and » do not correspond to word boundaries at either
// the character or byte level.
matiter!(unicode1, r"\bx\b", "«x", (2, 3));
matiter!(unicode2, r"\bx\b", "x»", (0, 1));