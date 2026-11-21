//! Fixed answers for the info buttons. Edit the text here; the robot speaks it exactly as written.
//! (No Ollama is used for these.)
//!
//! Tip: write things naturally ("C++", "C#", ".NET", phone and email). worker.rs converts
//! them to speakable words ("C plus plus", "C sharp", "dot net", digits) just before the voice is made.

pub struct Answer {
    /// Text on the button, also shown as the "question" in the panel
    pub label: &'static str,
    /// What the robot says (keep under about 1000 characters)
    pub text: &'static str,
}

pub static LEVELS: Answer = Answer {
    label: "Our 5 Levels",
    text: "AdaTech's Advanced Computer Programme has five levels, from beginner to professional readiness. \
Every level covers programming, 3D modelling, robotics and electronics, web or app development, game development, and a masterclass. \
Level 1, Beginner Foundations, covers first code, LEGO robots, and Tinkercad. \
Level 2, Intermediate Exploration, moves into Python with Pygame, apps, and websites. \
Level 3, Creative Technology, introduces C++, UI design, Roblox, and Godot. \
Level 4, Advanced Applications, covers C#, Unity, Flutter, and AI. \
Level 5, Professional Readiness, brings PyTorch, Unreal Engine, and a capstone portfolio. \
Book a free trial class and our team will recommend the right level for your child.",
};

pub static PROGRAMMING: Answer = Answer {
    label: "Programming Courses",
    text: "Programming grows with your child at every level. \
Level 1 starts with Python Turtle and Swift. \
Level 2 moves to Python with Pygame, plus drag and drop apps with Kodular. \
Level 3 introduces C++ with SFML, and Kotlin app development. \
Level 4 covers C# with .NET and WPF, Flutter apps, and AI with neural networks. \
Level 5 reaches advanced AI with PyTorch and OpenCV, React Native, and Java with JavaFX. \
Every level ends with real projects for a portfolio. Book a free trial class to see it in action.",
};

pub static MODELLING: Answer = Answer {
    label: "3D Modelling Courses",
    text: "3D modelling is part of every level. \
Level 1 starts with Tinkercad, where students design functional models and prepare them for 3D printing. \
Level 2 uses SketchUp for architecture and interiors. \
Level 3 moves to SelfCAD for mechanical parts and assemblies. \
Level 4 introduces Fusion 360 for precision CAD and motion simulation. \
Level 5 goes advanced with Blender and Unity for character modelling and animation. \
Students leave with real models they can present. Book a free trial class to try it.",
};

pub static GAMES: Answer = Answer {
    label: "Game Dev Courses",
    text: "Game development is one of our favourites. \
Level 1 uses Construct 3 to build 2D games with sprites, collisions, and sound. \
Level 2 adds Stencyl and GDevelop for multi-level platformers. \
Level 3 introduces Roblox Studio and Godot for 3D social games and platformer adventures. \
Level 4 moves to Unity for 3D games with AI characters. \
Level 5 finishes with Unity and Unreal Engine, covering VR, procedural worlds, multiplayer, and a capstone game project. \
Kids design the rules, levels, and characters themselves. Book a free trial class to start building.",
};

pub static ABOUT: Answer = Answer {
    label: "About Ada Tech",
    text: "AdaTech Institute is a technology institute in Dubai for ages 8 to 17, where students learn coding, robotics, electronics, AI, game development, and 3D modelling through hands-on projects. \
We are named after Ada Lovelace, the world's first computer programmer, who believed machines could create art, music, and ideas. \
ADA also stands for Art and Algorithms, Discovery and Design, and Ambition and Advancement. \
Our motto is Code as a Canvas. \
You can find us at Office 126, Block 13, Dubai Knowledge Park. \
Call or WhatsApp +971 52 560 8250, or email info@adatech.ae, to book a free trial class.",
};

/// All button answers (used to prepare their voices at startup)
pub fn all() -> [&'static Answer; 5] {
    [&LEVELS, &PROGRAMMING, &MODELLING, &GAMES, &ABOUT]
}
