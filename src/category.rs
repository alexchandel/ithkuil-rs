//! Grammatical categories in which formatives and adjucts are declined.

#[deriving(Show)]
pub enum Root {
	RootLiteral(String)
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Stem {
	Stem0 = 0,
	Stem1,
	Stem2
}

/// Holistic, Complementary 1, Complementary 2
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Pattern {
	Pattern0 = 0,
	Pattern1,
	Pattern2
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Function {
	Stative = 0,
	Dynamic,
	Manifestive,
	Descriptive
}

/// Physical relationship between members of noun.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Configuration {
	Uniplex = 0,
	Duplex,
	Discrete,
	Aggregative,
	Segmentative,
	Componential,
	Coherent,
	Composite,
	Multiform
}

/// Purpose, function, or benefit relationship between members.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Affiliation {
	Consolidative = 0,
	Associative,
	Variative,
	Coalescent
}

/// Manner in which noun/verb is spatially/temporally instantiated.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Perspective {
	Monadic = 0,
	Unbounded,
	Nomic,
	Abstract
}

/// The extent/boundaries of consideration of the noun/verb.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Extension {
	Delimitive = 0,
	Proximal,
	Inceptive,
	Terminative,
	Depletive,
	Graduative
}

/// Distinguishes real-world actualities from their alternative, imagined,
/// or potential counterparts.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Essence {
	Normal = 0,
	Representative
}

/// Psychologically implied fetures of formative.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Context {
	Existential = 0,
	Functional,
	Representational,
	Amalgamative
}

/// Authority, permanence, or contextual relevance.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Designation {
	Informal = 0,
	Formal
}

/// Participants to a verb.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum TransrelativeCase {
	Oblique = 0,
	Inducive,
	Absolutive,
	Ergative,
	Effectuative,
	Affective,
	Dative,
	Instrumental,
	Activative,
	Derivative,
	Situative
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum PossessiveCase {
	Proprietive = 0,
	Genitive,
	Attributive,
	Productive,
	Interpretative,
	Originative
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum AssociativeCase {
	Partitive = 0,
	Contrastive,
	Compositive,
	Predicative,
	Mediative,
	Applicative,
	Purposive,
	Considerative,
	Essive,
	Assimilative,
	Functive,
	Transformative,
	Referential,
	Classificative,
	Conductive,
	Interdependent,
	Benefactive,
	Transpositive,
	Commutative,
	Comitative,
	Conjunctive,
	Utilitative,
	Abessive,
	Conversive,
	Correlative,
	Dependent,
	Provisional,
	Postulative,
	Concessive,
	Exceptive,
	Aversive,
	Comparative
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum TemporalCase {
	Simultaneitive = 0,
	Assessive,
	Concursive,
	Accessive,
	Diffusive,
	Periodic,
	Prolapsive,
	Precursive,
	Postcursive,
	Elapsive,
	Allapsive,
	Interpolative,
	Episodic,
	Prolimitive,
	Limitative
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum SpatialCase {
	Locative = 0,
	Orientative,
	Procursive,
	Allative,
	Ablative,
	Navigative,
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum VocativeCase {
	Vocative = 0,
	ErrorCase
}

#[deriving(Show)]
pub enum Case {
	TransrelativeCases(TransrelativeCase),
	PossessiveCases(PossessiveCase),
	AssociativeCases(AssociativeCase),
	TemporalCases(TemporalCase),
	SpatialCases(SpatialCase),
	VocativeCases(VocativeCase)
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Format {
	NoFormat = 0,
	Schematic,
	Instrumentative,
	Objective,
	Authoritive,
	Precurrent,
	Resultative,
	Subsequent,
	Concommitant,
	Affinitive
}

/// Certainty of factuality of utterance, and of its assumption.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Mood {
	Factual = 0,
	Subjunctive,
	Assumptive,
	Speculative,
	Counterfactive,
	Hypothetical,
	Implicative,
	Ascriptive
}

/// Speech arts / general purpose of statement
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Illocution {
    Assertive = 0,
    Directive,
    Interrogative,
    Admonitive,
    Hortative,
    Declarative
}

/// Temporal pattern of action
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Phase {
	Contextual = 0,
	Punctual,
	Iterative,
	Repetitive,
	Intermittent,
	Recurrent,
	Frequentative,
	Fragmentative,
	Fluctuative,
}

/// Type of truthfulness of a statement
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Sanction {
	Propositional = 0,
	Epistemic,
	Allegative,
	Imputative,
	Refutative,
	Rebuttative,
	Theoretical,
	Expatiative,
	Axiomatic
}

/// Manner of participation
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Valence {
    Monoactive = 0,
    Parallel,
    Corollary,
    Reciprocal,
    Complementary,
    Nonrelational,
    Duplicative,
    Demonstrative,
    Resistive,
    Imitative,
    Contingent,
    Participative,
    Indicative,
    Mutual
}

/// Whether it's goal-oriented, and whether it's successful
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Version {
	Processual = 0,
	Completive,
	Ineffectual,
	Incompletive,
	Positive,
	Effective
}

/// Degree of type of evidence supporting statement.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Validation {
    Confirmative = 0,
    Affirmative,
    Reportive,
    Inferential,
    Intuitive,
    Presumptive,
    Presumptive2,
    Purportive,
    Purportive2,
    Conjectural,
    Dubitative,
    Tentative,
    Putative,
    Improbable
}

/// Temporal information related to "present" of action.
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Aspect {
	NoAspect = 0,
	Retrospective,
	Prospective,
	Habitual,
	Progressive,
	Imminent,
	Precessive,
	Regulative,
	Experiential,
	Resumptive,
	Cessative,
	Recessative,
	Pausal,
	Regressive,
	Preclusive,
	Continuative,
	Incessative,
	Preemptive,
	Climactic,
	Protractive,
	Temporary,
	Motive,
	Consequential,
	Sequential,
	Expeditive,
	Disclusive,
	Conclusive,
	Culminative,
	Intermediative,
	Tardative,
	Transitional,
	Intercommutative,
	Consumptive
}

/// Attitude of speaker
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Bias {
	NoBias = 0,
    Assurative,
    Hyperbolic,
    Coincidental,
    Acceptive,
    Reactive,
    Stupefactive,
    Contemplative,
    Desperative,
    Revelative,
    Gratificative,
    Solicitive,
    Selective,
    Ironic,
    Exasperative,
    Literal,
    Corrective,
    Euphemistic,
    Skeptical,
    Cynical,
    Contemptive,
    Dismissive,
    Indignative,
    Suggestive,
    Propositive
}

/// Same as modal verbs
#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Modality {
	NoModality = 0,
	Desiderative,
	Aspirative,
	Expective,
	Credential,
	Requisitive,
	Exhortative,
	Opportunistive,
	Capacitative,
	Permissive,
	Potential,
	Compulsory,
	Obligative,
	Impositive,
	Advocative,
	Intentive,
	Anticipative,
	Dispositive,
	Preparative,
	Necessitative,
	Decisive,
	Proclivitive,
	Voluntative,
	Accordative,
	Inclinative,
	Compulsive,
	Divertive,
	Devotive,
	Preferential,
	Impressional,
	Promissory
}

#[deriving(Show, FromPrimitive)]
#[repr(uint)]
pub enum Level {
	NoLevel = 0,
	Equative,
	Surpassive,
	Deficient,
	Optimal,
	Minimal,
	Superlative,
	Inferior,
	Superequative,
	Subequative
}

#[deriving(Show)] pub struct Cv(Sanction, Phase, Illocution);
#[deriving(Show)] pub struct Vl(Valence);
#[deriving(Show)] pub struct Cg(Validation);
#[deriving(Show)] pub struct Vr(Stem, Pattern, Function);
pub struct Cx(Root); // incorporated
pub struct Vp(Stem, Pattern, Designation); // incorporated
#[deriving(Show)] pub struct Cr(Root);
#[deriving(Show)] pub struct Vc(Case);
#[deriving(Show)] pub struct CiVi(Illocution, Mood);
#[deriving(Show)] pub struct Ca(Configuration, Affiliation, Perspective, Extension, Essence);
pub struct VxC(());
pub struct Vf(Context, Format);
pub struct Cb(Bias);
#[deriving(Show)] pub struct Tone(Version);
#[deriving(Show)] pub struct Stress(Designation);

#[deriving(Show)] pub struct Formative(Vr, Cr, Vc, Tone, Stress);

pub struct Cl(Valence);
pub struct Ve(Level);
// Cv
#[deriving(Show)] pub struct Vm(Modality);
#[deriving(Show)] pub struct Cs(Mood, Aspect);
#[deriving(Show)] pub struct Vs(Aspect);
// Cb

pub struct VerbalAdjunct(Vm, Cs);

/// Table of Sanction/Phase/(Illocution)
static tCv: &'static [&'static [&'static [&'static str]]] = &[
	&[&["t", "t’", "th", "l", "tr", "tl", "tř", "tw", "ty"],
	  &["k", "k’", "kh", "x", "kr", "kl", "kř", "kw", "ky"],
	  &["p", "p’", "ph", "vv", "pr", "pl", "př", "pw", "py"],
	  &["q", "q’", "qh", "ř", "qr", "ql", "xr", "qw", "xl"],
	  &["b", "v", "vr", "vl", "br", "bl", "bř", "bw", "by"],
	  &["d", "dh", "ż", "żż", "dr", "dl", "dř", "dw", "dy"],
	  &["g", "xh", "j", "jj", "gr", "gl", "gř", "gw", "gy"],
	  &["m", "mm", "r", "rr", "mr", "ml", "mř", "mw", "my"],
	  &["n", "nn", "ddh", "ll", "nr", "nl", "nř", "nw", "ny"]],
	&[&["s"],],
];

static tVl: &'static [&'static str] = &[
	"a", "e", "o", "i", "u",
	"â", "ê", "ô", "î/û",
	"ai/au", "ei/eu", "oi/ou", "ui/iu", "ö"
];

static tCg: &'static [&'static str] = &[
	"h", "y", "w", "hw", "hh",
	"hm", "hn", "hr", "lw", "ly", "rw", "ry", "řw", "řy"
];

/// Table of Stem/Pattern/Function vowels.
static tVr: &'static [&'static [&'static [&'static str]]] = &[
	&[&["a", "e", "u"],
	  &["o", "ö", "î/û"],
	  &["â", "ê", "ô"]],

	&[&["i", "ai", "ei"],
	  &["au", "eu", "iu"],
	  &["ia/ua", "ie/ue", "io/uo"]],

	&[&["ui", "ü/ou", "ëi"],
	  &["ae", "ea", "oa"],
	  &["üa/aì", "iù/uì", "iö/uö"]],

	&[&["oi", "eo", "eö"],
	  &["oe", "öe", "ëu"],
	  &["üo/oì", "üe/eì", "üö/aù"]],
];

/// Table of Case vowels.
static tVc: &'static [&'static str] = &[
	"a", "u", "e", "o", "ö", "i", "ü/a'e", "ai", "ei", "ui", "oi",

	"â", "î/û", "ê", "ô", "ëi", "öi", "ae",

	"ia/ua", "ie/ue", "io/uo", "iö/uö", "a'", "u'", "e'", "o'",
	"ea", "eo", "eö", "oa", "oe", "öa", "öe", "üa", "üe", "üo",
	"au", "eu", "iu", "ou", "ëu", "öu", "ai'", "ui'", "ei'", "oi'",
	"au'", "iu'", "eu'", "ou'",

	"a'a", "e'a", "i'a", "o'a", "u'a", "ö'a", "ü'a/ëu'a",
	"ai'a", "ai'a", "ei'a", "ui'a", "oi'a", "ëi'a",
	"au'a", "eu'a", "iu'a",

	"â'a", "ê'a", "ô'a", "î'a/û'a", "ëu'a", "ou'a",

	"ë"
];

/// Table of Mood/Illocution vowels.
static tCiVi: &'static [&'static [&'static str]] = &[
	&["wë/", "wa", "yë", "ya", "yû", "hë", "ha", "hû/hî"],
	&["we", "wö", "ye", "yö", "yeu/wei", "he", "hö", "hei"],
	&["wu", "wâ", "yu", "yâ", "yau/wai", "hu", "hâ", "hai"],
	&["wo", "wê", "yo", "yê", "you/woi", "ho", "hê", "hoi"],
	&["wi", "wô", "yi", "yô", "yiu/wui", "hi", "hô", "hui"],
	&["wî"]
];

/// Table of Configuration/Affiliation/Perspective/Extension/Essence consonants.
static tCa: &'static [&'static [&'static [&'static [&'static [&'static str]]]]] = &[
	// Normative shelf
	&[
		// Delimitive book
		&[
			// Monadic page
			&[&["l", "ll", "tļ", "ļ", "ļļ", "řļ", "sk", "šk", "kţ"],
			  &["r", "rr", "lļ", "rl", "rļ", "řl", "st", "št", "kç"],
			  &["m", "mm", "lm", "rm", "mļ", "řm", "sp", "šp", "pţ"],
			  &["n", "nn", "ln", "r n", "nļ", "ř n", "sq", "šq", "qţ"]],
			// Unbounded page
			&[&["t", "ļt", "lt", "rt", "nt", "řt", "ňt", "ňd", "çt"],
			  &["k", "ļk", "lk", "rk", "ňk", "řk", "kt", "xt", "çk"],
			  &["p", "ļp", "lp", "rp", "mp", "řp", "pt", "ft", "çp"],
			  &["q", "ļq", "lq", "rq", "ňq", "řq", "qt", "xht", "çq"]],
			// Nomic page
			&[&["ţ", "ţţ", "lţ", "rţ", "nţ", "řţ", "sţ", "šţ", "tf"],
			  &["x", "xx", "lx", "rx", "ňx", "mx", "sx", "šx", "kf"],
			  &["f", "ff", "lf", "rf", "mf", "řf", "sf", "šf", "pf"],
			  &["xh", "xxh", "lxh", "rxh", "ňxh", "mxh", "sxh", "šxh", "qf"]],
			// Abstract page
			&[&["c", "cc", "lc", "rc", "ns", "řc", "sc", "fk", "fq"],
			  &["č", "čč", "lč", "rč", "nš", "řč", "šč", "ţk", "ţq"],
			  &["ż", "żż", "lż", "rż", "nz", "řż", "ţf", "ţs", "ţc"],
			  &["j", "jj", "lj", "rj", "nž", "řj", "fţ", "ţš", "ţč"]]
		],
		// Proximal
		&[
			// Monadic page
			&[&["s",],
			  &["š",]],
			// Unbounded page
			&[&["d",],
			  &["g",]]
		]
	],

	// Representative shelf
	&[
		// Delimitive book
		&[
			// Monadic page
			&[&["tt",],
			  &["kk",]],
			// Unbounded page
			&[&["dd",],
			  &["gg",]]
		],
		// Proximal
		&[
			// Monadic page
			&[&["tw",],
			  &["kw",]],
			// Unbounded page
			&[&["ty",],
			  &["ky",]]
		]
	]
];

/// Table of modality vowels.
static tVm: &'static [&'static str] = &[
	"a", "u", "e", "o", "i",
	"ö", "î/û", "â", "ê", "ô", "ü/oe",
	"ai", "ei", "ui", "oi", "iu", "au", "eu", "ou", "ae",
	"ia/ua", "ie/ue", "io/uo", "iö/uö", "ea", "oa",
	"öi/ië", "öu/uë", "eo", "ëi", "ëu"
];

/// Table of (Mood)/Aspect consonants.
static tCs: &'static [&'static [&'static str]] = &[
	&["n-n", "m-m", "l-l", "r-r", "ň-ň", "r-n", "l-ň", "r-ň"],
	&["n-nr", "n-nt", "n-nt'", "n-nd", "n-nth", "n-nţ", "n-ndh", "n-nh"],
];

static tVs: &'static [&'static str] = &[
	"a", "u", "e", "o", "i",
	"ö", "î/û", "â", "ê", "ô", "ü/oe",
	"ai", "ei", "ui", "oi", "iu", "au", "eu", "ou", "ae",
	"ia/ua", "ie/ue", "io/uo", "iù/uì", "iö/uö", "ea", "oa", "eö",
	"öi/ië", "öu/uë", "eo", "ëi", "ëu"
];

pub fn encode_01(cv: Cv) -> &'static str {
	let Cv(sanction, phase, illocution) = cv;
	tCv	[illocution as uint]
		[phase as uint]
		[sanction as uint]
}

pub fn decode_01(cv: &str) -> Option<Cv> {
	for (illocution, page) in tCv.iter().enumerate() {
		for (phase, row) in page.iter().enumerate() {
			for (sanction, record) in row.iter().enumerate() {
				if record.contains(cv) {
					return Some(
						Cv(
							FromPrimitive::from_uint(sanction).unwrap(),
							FromPrimitive::from_uint(phase).unwrap(),
							FromPrimitive::from_uint(illocution).unwrap()
						)
					)
				}
			}
		}
	};
	None
}

pub fn encode_02(vl: Vl) -> &'static str {
	let Vl(valence) = vl;
	tVl[valence as uint]
}

pub fn decode_02(vl: &str) -> Option<Vl> {
	for (valence, record) in tVl.iter().enumerate() {
		if record.contains(vl) {
			return Some(
				Vl(
					FromPrimitive::from_uint(valence).unwrap()
				)
			)
		}
	};
	None
}

pub fn encode_03(cg: Cg) -> &'static str {
	let Cg(validation) = cg;
	tCg[validation as uint]
}

pub fn decode_03(cg: &str) -> Option<Cg> {
	for (validation, record) in tVl.iter().enumerate() {
		if record.contains(cg) {
			return Some(
				Cg(
					FromPrimitive::from_uint(validation).unwrap()
				)
			)
		}
	};
	None
}

pub fn encode_04(vr: Vr) -> &'static str {
	let Vr(stem, pattern, function) = vr;
	let si = stem as uint;
	let pi = pattern as uint;
	let fi = function as uint;
	tVr[fi][pi][si]
}

pub fn decode_04(vr: &str) -> Option<Vr> {
	for (function, page) in tVr.iter().enumerate() {
		for (pattern, row) in page.iter().enumerate() {
			for (stem, record) in row.iter().enumerate() {
				if record.contains(vr) {
					return Some(
						Vr(
							FromPrimitive::from_uint(stem).unwrap(),
							FromPrimitive::from_uint(pattern).unwrap(),
							FromPrimitive::from_uint(function).unwrap()
						)
					)
				}
			}
		}
	};
	None
}

pub fn encode_08(vc: Vc) -> &'static str {
	let Vc(case) = vc;
	let offset = match case {
		TransrelativeCases(c)	=> c as uint + 0u,
		PossessiveCases(c)		=> c as uint + 11u,
		AssociativeCases(c)		=> c as uint + 18u,
		TemporalCases(c)		=> c as uint + 50u,
		SpatialCases(c)			=> c as uint + 65u,
		VocativeCases(c)		=> c as uint + 71u
	};
	tVc[offset]
}

pub fn decode_08(vc: &str) -> Option<Vc> {
	for (ci, c) in tVc.iter().enumerate() {
		if c.contains(vc) {
			return Some(
				Vc(
					match ci {
						0..10 => TransrelativeCases(FromPrimitive::from_uint(ci).unwrap()),
						11..17 => PossessiveCases(FromPrimitive::from_uint(ci).unwrap()),
						18..49 => AssociativeCases(FromPrimitive::from_uint(ci).unwrap()),
						50..64 => TemporalCases(FromPrimitive::from_uint(ci).unwrap()),
						64..70 => SpatialCases(FromPrimitive::from_uint(ci).unwrap()),
						_ => VocativeCases(Vocative)
					}
				)
			)
		}
	};
	None
}

pub fn encode_09(civi: CiVi) -> &'static str {
	let CiVi(illocution, mood) = civi;
	match illocution {
		Declarative => match mood {
			Factual =>
				fail!("Declarative illocution can only be used with Factual mood"),
			_ => (),
		},
		_ => (),
	}
	tCiVi[illocution as uint][mood as uint]
}

pub fn decode_09(civi: &str) -> Option<CiVi> {
	for (illocution, row) in tCiVi.iter().enumerate() {
		for (mood, record) in row.iter().enumerate() {
			if record.contains(civi) {
				return Some(
					CiVi(
						FromPrimitive::from_uint(illocution).unwrap(),
						FromPrimitive::from_uint(mood).unwrap()
					)
				)
			}
		}
	};
	None
}

pub fn encode_10(ca: Ca) -> &'static str {
	let Ca(configuration, affiliation, perspective, extension, essence) = ca;
	tCa [essence as uint]
		[extension as uint]
		[perspective as uint]
		[affiliation as uint]
		[configuration as uint]
}

pub fn decode_10(ca: &str) -> Option<Ca> {
	for (essence, shelf) in tCa.iter().enumerate() {
		for (extension, book) in shelf.iter().enumerate() {
			for (perspective, page) in book.iter().enumerate() {
				for (affiliation, row) in page.iter().enumerate() {
					for (configuration, record) in row.iter().enumerate() {
						if record.contains(ca) {
							return Some (
								Ca(
									FromPrimitive::from_uint(configuration).unwrap(),
									FromPrimitive::from_uint(affiliation).unwrap(),
									FromPrimitive::from_uint(perspective).unwrap(),
									FromPrimitive::from_uint(extension).unwrap(),
									FromPrimitive::from_uint(essence).unwrap(),
								)
							)
						}
					}
				}
			}
		}
	}
	None
}

// pub fn decode_12(vf: &str) -> Option<Vf> {
// }

pub fn encode_d(vm: Vm) -> &'static str {
	let Vm(modality) = vm;
	tVm[modality as uint]
}

pub fn decode_d(vm: &str) -> Option<Vm> {
	for (modality, record) in tVm.iter().enumerate() {
		if record.contains(vm) {
			return Some(
				Vm(
					FromPrimitive::from_uint(modality).unwrap()
				)
			)
		}
	}
	None
}

pub fn encode_e(cs: Cs) -> &'static str {
	let Cs(mood, aspect) = cs;
	tCs[aspect as uint][mood as uint]
}

pub fn decode_e(cs: &str) -> Option<Cs> {
	for (aspect, row) in tCs.iter().enumerate() {
		for (mood, record) in row.iter().enumerate() {
			if record.contains(cs) {
				return Some(
					Cs(
						FromPrimitive::from_uint(mood).unwrap(),
						FromPrimitive::from_uint(aspect).unwrap()
					)
				)
			}
		}
	};
	None
}

pub fn encode_f(vs: Vs) -> &'static str {
	let Vs(aspect) = vs;
	tVs[aspect as uint]
}

pub fn decode_f(vs: &str) -> Option<Vs> {
	for (aspect, record) in tVs.iter().enumerate() {
		if record.contains(vs) {
			return Some(
				Vs(
					FromPrimitive::from_uint(aspect).unwrap()
				)
			)
		}
	};
	None
}
