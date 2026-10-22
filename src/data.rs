use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Greek,
    Math,
    Physics,
    Custom,
}

impl Category {
    pub const BUILTIN: [Category; 3] = [Category::Greek, Category::Math, Category::Physics];

    pub const ALL: [Category; 4] = [
        Category::Greek,
        Category::Math,
        Category::Physics,
        Category::Custom,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Category::Greek => "greek",
            Category::Math => "math",
            Category::Physics => "physics",
            Category::Custom => "custom",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "g" | "greek" | "alphabet" | "alphabets" => Some(Category::Greek),
            "m" | "math" | "mathematics" => Some(Category::Math),
            "p" | "physics" => Some(Category::Physics),
            "c" | "custom" | "user" => Some(Category::Custom),
            _ => None,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub ch: char,
    pub name: String,
    pub category: Category,
}

impl Symbol {
    fn new(ch: char, name: &str, category: Category) -> Self {
        Self {
            ch,
            name: name.to_string(),
            category,
        }
    }
}

const GREEK: &[(char, &str)] = &[
    ('α', "alpha"),
    ('β', "beta"),
    ('γ', "gamma"),
    ('δ', "delta"),
    ('ε', "epsilon"),
    ('ζ', "zeta"),
    ('η', "eta"),
    ('θ', "theta"),
    ('ι', "iota"),
    ('κ', "kappa"),
    ('λ', "lambda"),
    ('μ', "mu"),
    ('ν', "nu"),
    ('ξ', "xi"),
    ('ο', "omicron"),
    ('π', "pi"),
    ('ρ', "rho"),
    ('σ', "sigma"),
    ('τ', "tau"),
    ('υ', "upsilon"),
    ('φ', "phi"),
    ('χ', "chi"),
    ('ψ', "psi"),
    ('ω', "omega"),
];

const MATH: &[(char, &str)] = &[
    ('∀', "for all"),
    ('∃', "there exists"),
    ('∄', "there does not exist"),
    ('∅', "empty set"),
    ('∈', "element of"),
    ('∉', "not an element of"),
    ('⊂', "subset of"),
    ('⊃', "superset of"),
    ('⊆', "subset of or equal to"),
    ('⊇', "superset of or equal to"),
    ('∪', "union"),
    ('∩', "intersection"),
    ('∑', "n-ary summation"),
    ('∏', "n-ary product"),
    ('∐', "coproduct"),
    ('∫', "integral"),
    ('∬', "double integral"),
    ('∭', "triple integral"),
    ('∮', "contour integral"),
    ('∯', "surface integral"),
    ('∰', "volume integral"),
    ('∂', "partial differential"),
    ('∇', "nabla"),
    ('√', "square root"),
    ('∛', "cube root"),
    ('∜', "fourth root"),
    ('∞', "infinity"),
    ('≈', "approximately equal"),
    ('≉', "not approximately equal"),
    ('≠', "not equal"),
    ('≡', "identical to"),
    ('≤', "less than or equal to"),
    ('≥', "greater than or equal to"),
    ('≪', "much less than"),
    ('≫', "much greater than"),
    ('±', "plus-minus"),
    ('∓', "minus-plus"),
    ('×', "multiplication"),
    ('÷', "division"),
    ('⋅', "dot operator"),
    ('∘', "ring operator"),
    ('°', "degree"),
    ('′', "prime"),
    ('″', "double prime"),
    ('‴', "triple prime"),
    ('⁄', "fraction slash"),
    ('∧', "logical and"),
    ('∨', "logical or"),
    ('¬', "logical not"),
    ('⇒', "implies"),
    ('⇔', "if and only if"),
    ('→', "rightwards arrow"),
    ('←', "leftwards arrow"),
    ('↔', "left right arrow"),
    ('↦', "maps to"),
    ('∴', "therefore"),
    ('∵', "because"),
    ('∝', "proportional to"),
    ('∠', "angle"),
    ('⊥', "perpendicular"),
    ('∥', "parallel"),
    ('△', "triangle"),
    ('□', "square"),
    ('◇', "diamond"),
    ('ℓ', "script small l"),
    ('ℏ', "planck constant over two pi"),
    ('ℜ', "real part"),
    ('ℑ', "imaginary part"),
    ('℘', "weierstrass p"),
    ('⟨', "left angle bracket"),
    ('⟩', "right angle bracket"),
    ('⌈', "left ceiling"),
    ('⌉', "right ceiling"),
    ('⌊', "left floor"),
    ('⌋', "right floor"),
];

const PHYSICS: &[(char, &str)] = &[
    ('ℏ', "reduced planck constant"),
    ('ℎ', "planck constant"),
    ('ħ', "h-bar"),
    ('𝑐', "speed of light"),
    ('𝐺', "gravitational constant"),
    ('Å', "angstrom"),
    ('Ω', "ohm / omega baryon"),
    ('µ', "micro"),
    ('°', "degree"),
    ('′', "prime"),
    ('″', "double prime"),
    ('∂', "partial derivative"),
    ('∇', "nabla"),
    ('∆', "increment"),
    ('∞', "infinity"),
    ('∝', "proportional to"),
    ('≈', "approximately equal"),
    ('≡', "equivalent"),
    ('±', "plus-minus"),
    ('∓', "minus-plus"),
    ('×', "cross product"),
    ('·', "dot product"),
    ('→', "vector arrow"),
    ('⇌', "equilibrium"),
    ('⇋', "equilibrium harpoon"),
    ('⊥', "perpendicular"),
    ('∥', "parallel"),
    ('∠', "angle"),
    ('⊕', "circled plus"),
    ('⊗', "tensor product"),
    ('⊙', "circled dot"),
    ('⟨', "left angle bracket"),
    ('⟩', "right angle bracket"),
    ('∫', "integral"),
    ('∮', "contour integral"),
    ('∑', "summation"),
    ('∏', "product"),
    ('√', "square root"),
    ('⇀', "rightwards harpoon"),
    ('↼', "leftwards harpoon"),
    ('α', "alpha particle"),
    ('β', "beta particle"),
    ('γ', "photon"),
    ('ν', "neutrino"),
    ('Λ', "lambda baryon"),
    ('Σ', "sigma baryon"),
];

fn from_table(table: &[(char, &'static str)], category: Category) -> Vec<Symbol> {
    table
        .iter()
        .map(|&(ch, name)| Symbol::new(ch, name, category))
        .collect()
}

pub fn builtins() -> Vec<Symbol> {
    let mut symbols = from_table(GREEK, Category::Greek);
    symbols.extend(from_table(MATH, Category::Math));
    symbols.extend(from_table(PHYSICS, Category::Physics));
    symbols
}

pub fn for_category(category: Category) -> Vec<Symbol> {
    match category {
        Category::Greek => from_table(GREEK, Category::Greek),
        Category::Math => from_table(MATH, Category::Math),
        Category::Physics => from_table(PHYSICS, Category::Physics),
        Category::Custom => Vec::new(),
    }
}
