use super::*;

/* Tests for creating, loading, unloading,
and interacting with blocks from different files. */
mod blocks_tests;
/* Tests for highlighting text and deleting
said highlight. */
mod selection_tests;
/* Tests for the key_functions module. */
mod key_functions_tests;

/*
========================================
			USEFUL FUNCTIONS
========================================
*/

// Construct and initialize and EditorSpace for the given file
fn construct_editor(filename: &str) -> EditorSpace {
	// Create a default config
	let config = Config::default();
	// Create an EditorSpace over the specified file
	let mut editor = EditorSpace::new(String::from(filename), config);
	// Initialize the editor (which constructs a single TextBlock Blocks)
	let _ = editor.init_editor((0, 0), 500, 50);

	editor
}

// Get the content of all blocks in a Blocks
fn get_content(blocks: Blocks) -> Vec<String> {
	// Vector of all lines of text in the Blocks
	let mut content = Vec::new();
	// Get the lines of text from blocks
	for block in blocks.blocks_list {
		content.extend(block.content);
	}

	content
}

/*
==========================================
			FILENAME CONSTANTS
==========================================
*/

// Decent sized text file
const GENOME_FILE: &str = "../editor/test_files/GRCh38_50_rna.fna";

// Text file that is smaller than a widget
const SMALL_FILE: &str = "../editor/test_files/small_text.txt";

// Text file used to test basic highlighting functionality
const HIGHLIGHT_FILE: &str = "../editor/test_files/highlight.txt";

/*
=============================================
			GENOME FILE CONSTANTS
=============================================
*/

// First block of the Genome File
const GENOME_BLOCK_1: &str =
	">NM_000014.6 Homo sapiens alpha-2-macroglobulin (A2M), transcript variant 1, mRNA\n\
    GGGACCAGATGGATTGTAGGGAGTAGGGTACAATACAGTCTGTTCTCCTCCAGCTCCTTCTTTCTGCAACATGGGGAAGA\n\
    ACAAACTCCTTCATCCAAGTCTGGTTCTTCTCCTCTTGGTCCTCCTGCCCACAGACGCCTCAGTCTCTGGAAAACCGCAG\n\
    TATATGGTTCTGGTCCCCTCCCTGCTCCACACTGAGACCACTGAGAAGGGCTGTGTCCTTCTGAGCTACCTGAATGAGAC\n\
    AGTGACTGTAAGTGCTTCCTTGGAGTCTGTCAGGGGAAACAGGAGCCTCTTCACTGACCTGGAGGCGGAGAATGACGTAC\n\
    TCCACTGTGTCGCCTTCGCTGTCCCAAAGTCTTCATCCAATGAGGAGGTAATGTTCCTCACTGTCCAAGTGAAAGGACCA\n\
    ACCCAAGAATTTAAGAAGCGGACCACAGTGATGGTTAAGAACGAGGACAGTCTGGTCTTTGTCCAGACAGACAAATCAAT\n\
    CTACAAACCAGGGCAGACAGTGAAATTTCGTGTTGTCTCCATGGATGAAAACTTTCACCCCCTGAATGAGTTGATTCCAC\n\
    TAGTATACATTCAGGATCCCAAAGGAAATCGCATCGCACAATGGCAGAGTTTCCAGTTAGAGGGTGGCCTCAAGCAATTT\n\
    TCTTTTCCCCTCTCATCAGAGCCCTTCCAGGGCTCCTACAAGGTGGTGGTACAGAAGAAATCAGGTGGAAGGACAGAGCA\n\
    CCCTTTCACCGTGGAGGAATTTGTTCTTCCCAAGTTTGAAGTACAAGTAACAGTGCCAAAGATAATCACCATCTTGGAAG\n\
    AAGAGATGAATGTATCAGTGTGTGGCCTATACACATATGGGAAGCCTGTCCCTGGACATGTGACTGTGAGCATTTGCAGA\n\
    AAGTATAGTGACGCTTCCGACTGCCACGGTGAAGATTCACAGGCTTTCTGTGAGAAATTCAGTGGACAGCTAAACAGCCA\n\
    TGGCTGCTTCTATCAGCAAGTAAAAACCAAGGTCTTCCAGCTGAAGAGGAAGGAGTATGAAATGAAACTTCACACTGAGG\n\
    CCCAGATCCAAGAAGAAGGAACAGTGGTGGAATTGACTGGAAGGCAGTCCAGTGAAATCACAAGAACCATAACCAAACTC\n\
    TCATTTGTGAAAGTGGACTCACACTTTCGACAGGGAATTCCCTTCTTTGGGCAGGTGCGCCTAGTAGATGGGAAAGGCGT\n\
    CCCTATACCAAATAAAGTCATATTCATCAGAGGAAATGAAGCAAACTATTACTCCAATGCTACCACGGATGAGCATGGCC\n\
    TTGTACAGTTCTCTATCAACACCACCAATGTTATGGGTACCTCTCTTACTGTTAGGGTCAATTACAAGGATCGTAGTCCC\n\
    TGTTACGGCTACCAGTGGGTGTCAGAAGAACACGAAGAGGCACATCACACTGCTTATCTTGTGTTCTCCCCAAGCAAGAG\n\
    CTTTGTCCACCTTGAGCCCATGTCTCATGAACTACCCTGTGGCCATACTCAGACAGTCCAGGCACATTATATTCTGAATG\n\
    GAGGCACCCTGCTGGGGCTGAAGAAGCTCTCCTTCTATTATCTGATAATGGCAAAGGGAGGCATTGTCCGAACTGGGACT\n\
    CATGGACTGCTTGTGAAGCAGGAAGACATGAAGGGCCATTTTTCCATCTCAATCCCTGTGAAGTCAGACATTGCTCCTGT\n\
    CGCTCGGTTGCTCATCTATGCTGTTTTACCTACCGGGGACGTGATTGGGGATTCTGCAAAATATGATGTTGAAAATTGTC\n\
    TGGCCAACAAGGTGGATTTGAGCTTCAGCCCATCACAAAGTCTCCCAGCCTCACACGCCCACCTGCGAGTCACAGCGGCT\n\
    CCTCAGTCCGTCTGCGCCCTCCGTGCTGTGGACCAAAGCGTGCTGCTCATGAAGCCTGATGCTGAGCTCTCGGCGTCCTC\n\
    GGTTTACAACCTGCTACCAGAAAAGGACCTCACTGGCTTCCCTGGGCCTTTGAATGACCAGGACAATGAAGACTGCATCA\n\
    ATCGTCATAATGTCTATATTAATGGAATCACATATACTCCAGTATCAAGTACAAATGAAAAGGATATGTACAGCTTCCTA\n\
    GAGGACATGGGCTTAAAGGCATTCACCAACTCAAAGATTCGTAAACCCAAAATGTGTCCACAGCTTCAACAGTATGAAAT\n\
    GCATGGACCTGAAGGTCTACGTGTAGGTTTTTATGAGTCAGATGTAATGGGAAGAGGCCATGCACGCCTGGTGCATGTTG\n\
    AAGAGCCTCACACGGAGACCGTACGAAAGTACTTCCCTGAGACATGGATCTGGGATTTGGTGGTGGTAAACTCAGCAGGT\n\
    GTGGCTGAGGTAGGAGTAACAGTCCCTGACACCATCACCGAGTGGAAGGCAGGGGCCTTCTGCCTGTCTGAAGATGCTGG\n\
    ACTTGGTATCTCTTCCACTGCCTCTCTCCGAGCCTTCCAGCCCTTCTTTGTGGAGCTCACAATGCCTTACTCTGTGATTC\n\
    GTGGAGAGGCCTTCACACTCAAGGCCACGGTCCTAAACTACCTTCCCAAATGCATCCGGGTCAGTGTGCAGCTGGAAGCC\n\
    TCTCCCGCCTTCCTAGCTGTCCCAGTGGAGAAGGAACAAGCGCCTCACTGCATCTGTGCAAACGGGCGGCAAACTGTGTC\n\
    CTGGGCAGTAACCCCAAAGTCATTAGGAAATGTGAATTTCACTGTGAGCGCAGAGGCACTAGAGTCTCAAGAGCTGTGTG\n\
    GGACTGAGGTGCCTTCAGTTCCTGAACACGGAAGGAAAGACACAGTCATCAAGCCTCTGTTGGTTGAACCTGAAGGACTA\n\
    GAGAAGGAAACAACATTCAACTCCCTACTTTGTCCATCAGGTGGTGAGGTTTCTGAAGAATTATCCCTGAAACTGCCACC\n\
    AAATGTGGTAGAAGAATCTGCCCGAGCTTCTGTCTCAGTTTTGGGAGACATATTAGGCTCTGCCATGCAAAACACACAAA\n\
    ATCTTCTCCAGATGCCCTATGGCTGTGGAGAGCAGAATATGGTCCTCTTTGCTCCTAACATCTATGTACTGGATTATCTA\n\
    AATGAAACACAGCAGCTTACTCCAGAGATCAAGTCCAAGGCCATTGGCTATCTCAACACTGGTTACCAGAGACAGTTGAA\n\
    CTACAAACACTATGATGGCTCCTACAGCACCTTTGGGGAGCGATATGGCAGGAACCAGGGCAACACCTGGCTCACAGCCT\n\
    TTGTTCTGAAGACTTTTGCCCAAGCTCGAGCCTACATCTTCATCGATGAAGCACACATTACCCAAGCCCTCATATGGCTC\n\
    TCCCAGAGGCAGAAGGACAATGGCTGTTTCAGGAGCTCTGGGTCACTGCTCAACAATGCCATAAAGGGAGGAGTAGAAGA\n\
    TGAAGTGACCCTCTCCGCCTATATCACCATCGCCCTTCTGGAGATTCCTCTCACAGTCACTCACCCTGTTGTCCGCAATG\n\
    CCCTGTTTTGCCTGGAGTCAGCCTGGAAGACAGCACAAGAAGGGGACCATGGCAGCCATGTATATACCAAAGCACTGCTG\n\
    GCCTATGCTTTTGCCCTGGCAGGTAACCAGGACAAGAGGAAGGAAGTACTCAAGTCACTTAATGAGGAAGCTGTGAAGAA\n\
    AGACAACTCTGTCCATTGGGAGCGCCCTCAGAAACCCAAGGCACCAGTGGGGCATTTTTACGAACCCCAGGCTCCCTCTG\n\
    CTGAGGTGGAGATGACATCCTATGTGCTCCTCGCTTATCTCACGGCCCAGCCAGCCCCAACCTCGGAGGACCTGACCTCT\n\
    GCAACCAACATCGTGAAGTGGATCACGAAGCAGCAGAATGCCCAGGGCGGTTTCTCCTCCACCCAGGACACAGTGGTGGC\n\
    TCTCCATGCTCTGTCCAAATATGGAGCAGCCACATTTACCAGGACTGGGAAGGCTGCACAGGTGACTATCCAGTCTTCAG\n\
    GGACATTTTCCAGCAAATTCCAAGTGGACAACAACAACCGCCTGTTACTGCAGCAGGTCTCATTGCCAGAGCTGCCTGGG\n\
    GAATACAGCATGAAAGTGACAGGAGAAGGATGTGTCTACCTCCAGACATCCTTGAAATACAATATTCTCCCAGAAAAGGA\n\
    AGAGTTCCCCTTTGCTTTAGGAGTGCAGACTCTGCCTCAAACTTGTGATGAACCCAAAGCCCACACCAGCTTCCAAATCT\n\
    CCCTAAGTGTCAGTTACACAGGGAGCCGCTCTGCCTCCAACATGGCGATCGTTGATGTGAAGATGGTCTCTGGCTTCATT\n\
    CCCCTGAAGCCAACAGTGAAAATGCTTGAAAGATCTAACCATGTGAGCCGGACAGAAGTCAGCAGCAACCATGTCTTGAT\n\
    TTACCTTGATAAGGTGTCAAATCAGACACTGAGCTTGTTCTTCACGGTTCTGCAAGATGTCCCAGTAAGAGATCTGAAAC\n\
    CAGCCATAGTGAAAGTCTATGATTACTACGAGACGGATGAGTTTGCAATTGCTGAGTACAATGCTCCTTGCAGCAAAGAT\n\
    CTTGGAAATGCTTGAAGACCACAAGGCTGAAAAGTGCTTTGCTGGAGTCCTGTTCTCAGAGCTCCACAGAAGACACGTGT\n\
    TTTTGTATCTTTAAAGACTTGATGAATAAACACTTTTTCTGGTCAATGTC\n\
    >NM_000015.3 Homo sapiens N-acetyltransferase 2 (NAT2), mRNA\n\
    ACTTTATTACAGACCTTGGAAGCAAGAGGATTGCATTCAGCCTAGTTCCTGGTTGCTGGCCAAAGGGATCATGGACATTG\n\
    AAGCATATTTTGAAAGAATTGGCTATAAGAACTCTAGGAACAAATTGGACTTGGAAACATTAACTGACATTCTTGAGCAC\n\
    CAGATCCGGGCTGTTCCCTTTGAGAACCTTAACATGCATTGTGGGCAAGCCATGGAGTTGGGCTTAGAGGCTATTTTTGA";

// Second block of the Genome File
const GENOME_BLOCK_2: &str = "TCACATTGTAAGAAGAAACCGGGGTGGGTGGTGTCTCCAGGTCAATCAACTTCTGTACTGGGCTCTGACCACAATCGGTT\n\
    TTCAGACCACAATGTTAGGAGGGTATTTTTACATCCCTCCAGTTAACAAATACAGCACTGGCATGGTTCACCTTCTCCTG\n\
    CAGGTGACCATTGACGGCAGGAATTACATTGTCGATGCTGGGTCTGGAAGCTCCTCCCAGATGTGGCAGCCTCTAGAATT\n\
    AATTTCTGGGAAGGATCAGCCTCAGGTGCCTTGCATTTTCTGCTTGACAGAAGAGAGAGGAATCTGGTACCTGGACCAAA\n\
    TCAGGAGAGAGCAGTATATTACAAACAAAGAATTTCTTAATTCTCATCTCCTGCCAAAGAAGAAACACCAAAAAATATAC\n\
    TTATTTACGCTTGAACCTCGAACAATTGAAGATTTTGAGTCTATGAATACATACCTGCAGACGTCTCCAACATCTTCATT\n\
    TATAACCACATCATTTTGTTCCTTGCAGACCCCAGAAGGGGTTTACTGTTTGGTGGGCTTCATCCTCACCTATAGAAAAT\n\
    TCAATTATAAAGACAATACAGATCTGGTCGAGTTTAAAACTCTCACTGAGGAAGAGGTTGAAGAAGTGCTGAGAAATATA\n\
    TTTAAGATTTCCTTGGGGAGAAATCTCGTGCCCAAACCTGGTGATGGATCCCTTACTATTTAGAATAAGGAACAAAATAA\n\
    ACCCTTGTGTATGTATCACCCAACTCACTAATTATCAACTTATGTGCTATCAGATATCCTCTCTACCCTCACGTTATTTT\n\
    GAAGAAAATCCTAAACATCAAATACTTTCATCCATAAAAATGTCAGCATTTATTAAAAAACAATAACTTTTTAAAGAAAC\n\
    ATAAGGACACATTTTCAAATTAATAAAAATAAAGGCATTTTAAGGATGGCCTGTGATTATCTTGGGAAGCAGAGTGATTC\n\
    ATGCTAGAAAACATTTAATATTGATTTATTGTTGAATTCATAGTAAATTTTTACTGGTAAATGAATAAAGAATATTGTGG\n\
    AAAAA\n\
    >NM_000016.6 Homo sapiens acyl-CoA dehydrogenase medium chain (ACADM), transcript variant 1, mRNA; nuclear gene for mitochondrial product\n\
    AGAGGAGTCCCGCGTTCGGGGAGTATGTCAAGGCCGTGACCCGTGTATTATTGTCCGAGTGGCCGGAACGGGAGCCAACA\n\
    TGGCAGCGGGGTTCGGGCGATGCTGCAGGGTCCTGAGAAGTATTTCTCGTTTTCATTGGAGATCACAGCATACAAAAGCC\n\
    AATCGACAACGTGAACCAGGATTAGGATTTAGTTTTGAGTTCACCGAACAGCAGAAAGAATTTCAAGCTACTGCTCGTAA\n\
    ATTTGCCAGAGAGGAAATCATCCCAGTGGCTGCAGAATATGATAAAACTGGTGAATATCCAGTCCCCCTAATTAGAAGAG\n\
    CCTGGGAACTTGGTTTAATGAACACACACATTCCAGAGAACTGTGGAGGTCTTGGACTTGGAACTTTTGATGCTTGTTTA\n\
    ATTAGTGAAGAATTGGCTTATGGATGTACAGGGGTTCAGACTGCTATTGAAGGAAATTCTTTGGGGCAAATGCCTATTAT\n\
    TATTGCTGGAAATGATCAACAAAAGAAGAAGTATTTGGGGAGAATGACTGAGGAGCCATTGATGTGTGCTTATTGTGTAA\n\
    CAGAACCTGGAGCAGGCTCTGATGTAGCTGGTATAAAGACCAAAGCAGAAAAGAAAGGAGATGAGTATATTATTAATGGT\n\
    CAGAAGATGTGGATAACCAACGGAGGAAAAGCTAATTGGTATTTTTTATTGGCACGTTCTGATCCAGATCCTAAAGCTCC\n\
    TGCTAATAAAGCCTTTACTGGATTCATTGTGGAAGCAGATACCCCAGGAATTCAGATTGGGAGAAAGGAATTAAACATGG\n\
    GCCAGCGATGTTCAGATACTAGAGGAATTGTCTTCGAAGATGTGAAAGTGCCTAAAGAAAATGTTTTAATTGGTGACGGA\n\
    GCTGGTTTCAAAGTTGCAATGGGAGCTTTTGATAAAACCAGACCTGTAGTAGCTGCTGGTGCTGTTGGATTAGCACAAAG\n\
    AGCTTTGGATGAAGCTACCAAGTATGCCCTGGAAAGGAAAACTTTCGGAAAGCTACTTGTAGAGCACCAAGCAATATCAT\n\
    TTATGCTGGCTGAAATGGCAATGAAAGTTGAACTAGCTAGAATGAGTTACCAGAGAGCAGCTTGGGAGGTTGATTCTGGT\n\
    CGTCGAAATACCTATTATGCTTCTATTGCAAAGGCATTTGCTGGAGATATTGCAAATCAGTTAGCTACTGATGCTGTGCA\n\
    GATACTTGGAGGCAATGGATTTAATACAGAATATCCTGTAGAAAAACTAATGAGGGATGCCAAAATCTATCAGATTTATG\n\
    AAGGTACTTCACAAATTCAAAGACTTATTGTAGCCCGTGAACACATTGACAAGTACAAAAATTAAAAAAATTACTGTAGA\n\
    AATATTGAATAACTAGAACACAAGCCACTGTTTCAGCTCCAGAAAAAAGAAAGGGCTTTAACGTTTTTTCCAGTGAAAAC\n\
    AAATCCTCTTATATTAAATCTAAGCAACTGCTTATTATAGTAGTTTATACTTTTGCTTAACTCTGTTATGTCTCTTAAGC\n\
    AGGTTTGGTTTTTATTAAAATGATGTGTTTTCTTTAGTACCACTTTACTTGAATTACATTAACCTAGAAAACTACATAGG\n\
    TTATTTTGATCTCTTAAGATTAATGTAGCAGAAATTTCTTGGAATTTTATTTTTGTAATGACAGAAAAGTGGGCTTAGAA\n\
    AGTATTCAAGATGTTACAAAATTTACATTTAGAAAATATTGTAGTATTTGAATACTGTCAACTTGACAGTAACTTTGTAG\n\
    ACTTAATGGTATTATTAAAGTTCTTTTTATTGCAGTTTGGAAAGCATTTGTGAAACTTTCTGTTTGGCACAGAAACAGTC\n\
    AAAATTTTGACATTCATATTCTCCTATTTTACAGCTACAAGAACTTTCTTGAAAATCTTATTTAATTCTGAGCCCATATT\n\
    TCACTTACCTTATTTAAAATAAATCAATAAAGCTTGCCTTAAATTATTTTTATATGACTGTTGGTCTCTAGGTAGCCTTT\n\
    GGTCTATTGTACACAATCTCATTTCATATGTTTGCATTTTGGCAAAGAACTTAATAAAATTGTTCAGTGCTTATTATCAT\n\
    ATCTTTCTGTATTTTTTCCAGGAAATTTCATTACTTCGTGTAATAGTGTATATTTCTTGTATTTACTATGATGAAAAAAG\n\
    GTCGTTTTAATTTTGAATTGAATAAAGTTACCTGTTCATTTTTTATTAGATATTTTAAAGACTTCAGAAAATATAAATAT\n\
    GAAATAATTTAAGAACCCAAA\n\
    >NM_000017.4 Homo sapiens acyl-CoA dehydrogenase short chain (ACADS), transcript variant 1, mRNA; nuclear gene for mitochondrial product\n\
    ACTCCGGAACAGCGCGCTCGCAGCGGGAGGTCGCGAAGCCTGGGACTGTGTCTGTCGCCCATGGCCGCCGCGCTGCTCGC\n\
    CCGGGCCTCGGGCCCTGCCCGCAGAGCTCTCTGTCCTAGGGCCTGGCGGCAGTTACACACCATCTACCAGTCTGTGGAAC\n\
    TGCCCGAGACACACCAGATGTTGCTCCAGACATGCCGGGACTTTGCCGAGAAGGAGTTGTTTCCCATTGCAGCCCAGGTG\n\
    GATAAGGAACATCTCTTCCCAGCGGCTCAGGTGAAGAAGATGGGCGGGCTTGGGCTTCTGGCCATGGACGTGCCCGAGGA\n\
    GCTTGGCGGTGCTGGCCTCGATTACCTGGCCTACGCCATCGCCATGGAGGAGATCAGCCGTGGCTGCGCCTCCACCGGAG\n\
    TCATCATGAGTGTCAACAACTCTCTCTACCTGGGGCCCATCTTGAAGTTTGGCTCCAAGGAGCAGAAGCAGGCGTGGGTC\n\
    ACGCCTTTCACCAGTGGTGACAAAATTGGCTGCTTTGCCCTCAGCGAACCAGGGAACGGCAGTGATGCAGGAGCTGCGTC\n\
    CACCACCGCCCGGGCCGAGGGCGACTCATGGGTTCTGAATGGAACCAAAGCCTGGATCACCAATGCCTGGGAGGCTTCGG\n\
    CTGCCGTGGTCTTTGCCAGCACGGACAGAGCCCTGCAAAACAAGGGCATCAGTGCCTTCCTGGTCCCCATGCCAACGCCT\n\
    GGGCTCACGTTGGGGAAGAAAGAAGACAAGCTGGGCATCCGGGGCTCATCCACGGCCAACCTCATCTTTGAGGACTGTCG\n\
    CATCCCCAAGGACAGCATCCTGGGGGAGCCAGGGATGGGCTTCAAGATAGCCATGCAAACCCTGGACATGGGCCGCATCG\n\
    GCATCGCCTCCCAGGCCCTGGGCATTGCCCAGACCGCCCTCGATTGTGCTGTGAACTACGCTGAGAATCGCATGGCCTTC\n\
    GGGGCGCCCCTCACCAAGCTCCAGGTCATCCAGTTCAAGTTGGCAGACATGGCCCTGGCCCTGGAGAGTGCCCGGCTGCT\n\
    GACCTGGCGCGCTGCCATGCTGAAGGATAACAAGAAGCCTTTCATCAAGGAGGCAGCCATGGCCAAGCTGGCCGCCTCGG\n\
    AGGCCGCGACCGCCATCAGCCACCAGGCCATCCAGATCCTGGGCGGCATGGGCTACGTGACAGAGATGCCGGCAGAGCGG\n\
    CACTACCGCGACGCCCGCATCACTGAGATCTACGAGGGCACCAGCGAAATCCAGCGGCTGGTGATCGCCGGGCATCTGCT\n\
    CAGGAGCTACCGGAGCTGAGCCCGCGGCGGACTGCCCCAGGACTGCGGGAAGGCGCGGGAGCCAGGGGCCTCCACCCCAA\n\
    CCCCGGCTCAGAGACTGGGCGGCCCGGCGGGGGCTCCCTGGGGACCCCAGATGGGCTCAGTGCTGCCACCCAGATCAGAT\n\
    CACATGGGAATGAGGCCCTCCGACCATTGGCAGCTCCGCCTCTGGGCCTTTCCGCCTCCTCACCACTGTGCCTCAAGTTC";

// Third block of the Genome File
const GENOME_BLOCK_3: &str = "CTCATCTAAGTGGCCCTGGCCTCCTGGGGGCGGGGTTGTGGGGGGGCTGAGCGACACTCAGGGACACCTCAGTTGTCCTC\n\
    CCGCGGGCCCTGGTGCCCTGGCATGAAGGCCCAGTGCGACAGGCCCTTGGTGGGGTCTGTCTTTTCCTTGAGGTCAGAGG\n\
    TCAGGAGCAGGGCTGGGGTCAGGATGACGAGGCCTGGGGTCCTGGTGTTGGGCAGGTGGTGGGGCTGGGCCATGGAGCTG\n\
    GCCCAGAGGCCCCTCAGCCCTTTGTAAAGTCTGATGAAGGCAGGGGTGGTGATTCATGCTGTGTGACTGACTGTGGGTAA\n\
    TAAACACACCTGTCCCCCA\n\
    >NM_000018.4 Homo sapiens acyl-CoA dehydrogenase very long chain (ACADVL), transcript variant 1, mRNA; nuclear gene for mitochondrial product\n\
    AGAGCTGGGTCAGAGCTCGAGCCAGCGGCGCCCGGAGAGATTCGGAGATGCAGGCGGCTCGGATGGCCGCGAGCTTGGGG\n\
    CGGCAGCTGCTGAGGCTCGGGGGCGGAAGCTCGCGGCTCACGGCGCTCCTGGGGCAGCCCCGGCCCGGCCCTGCCCGGCG\n\
    GCCCTATGCCGGGGGTGCCGCTCAGCTGGCTCTGGACAAGTCAGATTCCCACCCCTCTGACGCTCTGACCAGGAAAAAAC\n\
    CGGCCAAGGCGGAATCTAAGTCCTTTGCTGTGGGAATGTTCAAAGGCCAGCTCACCACAGATCAGGTGTTCCCATACCCG\n\
    TCCGTGCTCAACGAAGAGCAGACACAGTTTCTTAAAGAGCTGGTGGAGCCTGTGTCCCGTTTCTTCGAGGAAGTGAACGA\n\
    TCCCGCCAAGAATGACGCTCTGGAGATGGTGGAGGAGACCACTTGGCAGGGCCTCAAGGAGCTGGGGGCCTTTGGTCTGC\n\
    AAGTGCCCAGTGAGCTGGGTGGTGTGGGCCTTTGCAACACCCAGTACGCCCGTTTGGTGGAGATCGTGGGCATGCATGAC\n\
    CTTGGCGTGGGCATTACCCTGGGGGCCCATCAGAGCATCGGTTTCAAAGGCATCCTGCTCTTTGGCACAAAGGCCCAGAA\n\
    AGAAAAATACCTCCCCAAGCTGGCATCTGGGGAGACTGTGGCCGCTTTCTGTCTAACCGAGCCCTCAAGCGGGTCAGATG\n\
    CAGCCTCCATCCGAACCTCTGCTGTGCCCAGCCCCTGTGGAAAATACTATACCCTCAATGGAAGCAAGCTTTGGATCAGT\n\
    AATGGGGGCCTAGCAGACATCTTCACGGTCTTTGCCAAGACACCAGTTACAGATCCAGCCACAGGAGCCGTGAAGGAGAA\n\
    GATCACAGCTTTTGTGGTGGAGAGGGGCTTCGGGGGCATTACCCATGGGCCCCCTGAGAAGAAGATGGGCATCAAGGCTT\n\
    CAAACACAGCAGAGGTGTTCTTTGATGGAGTACGGGTGCCATCGGAGAACGTGCTGGGTGAGGTTGGGAGTGGCTTCAAG\n\
    GTTGCCATGCACATCCTCAACAATGGAAGGTTTGGCATGGCTGCGGCCCTGGCAGGTACCATGAGAGGCATCATTGCTAA\n\
    GGCGGTAGATCATGCCACTAATCGTACCCAGTTTGGGGAGAAAATTCACAACTTTGGGCTGATCCAGGAGAAGCTGGCAC\n\
    GGATGGTTATGCTGCAGTATGTAACTGAGTCCATGGCTTACATGGTGAGTGCTAACATGGACCAGGGAGCCACGGACTTC\n\
    CAGATAGAGGCCGCCATCAGCAAAATCTTTGGCTCGGAGGCAGCCTGGAAGGTGACAGATGAATGCATCCAAATCATGGG\n\
    GGGTATGGGCTTCATGAAGGAACCTGGAGTAGAGCGTGTGCTCCGAGATCTTCGCATCTTCCGGATCTTTGAGGGGACAA\n\
    ATGACATTCTTCGGCTGTTTGTGGCTCTGCAGGGCTGTATGGACAAAGGAAAGGAGCTCTCTGGGCTTGGCAGTGCTCTA\n\
    AAGAATCCCTTTGGGAATGCTGGCCTCCTGCTAGGAGAGGCAGGCAAACAGCTGAGGCGGCGGGCAGGGCTGGGCAGCGG\n\
    CCTGAGTCTCAGCGGACTTGTCCACCCGGAGTTGAGTCGGAGTGGCGAGCTGGCAGTACGGGCTCTGGAGCAGTTTGCCA\n\
    CTGTGGTGGAGGCCAAGCTGATAAAACACAAGAAGGGGATTGTCAATGAACAGTTTCTGCTGCAGCGGCTGGCAGACGGG\n\
    GCCATCGACCTCTATGCCATGGTGGTGGTTCTCTCGAGGGCCTCAAGATCCCTGAGTGAGGGCCACCCCACGGCCCAGCA\n\
    TGAGAAAATGCTCTGTGACACCTGGTGTATCGAGGCTGCAGCTCGGATCCGAGAGGGCATGGCCGCCCTGCAGTCTGACC\n\
    CCTGGCAGCAAGAGCTCTACCGCAACTTCAAAAGCATCTCCAAGGCCTTGGTGGAGCGGGGTGGTGTGGTCACCAGCAAC\n\
    CCACTTGGCTTCTGAATACTCCCGGCCAGGGCCTGTCCCAGTTATGTGCCTTCCCTCAAGCCAAAGCCGAAGCCCCTTTC\n\
    CTTAAGGCCCTGGTTTGTCCCGAAGGGGCCTAGTGTTCCCAGCACTGTGCCTGCTCTCAAGAGCACTTACTGCCTCGCAA\n\
    ATAATAAAAATTTCTAGCCAGTCA\n\
    >NM_000019.4 Homo sapiens acetyl-CoA acetyltransferase 1 (ACAT1), transcript variant 2, mRNA; nuclear gene for mitochondrial product\n\
    AGTCTACGCCTGTGGAGCCGATACTCAGCCCTCTGCGACCATGGCTGTGCTGGCGGCACTTCTGCGCAGCGGCGCCCGCA\n\
    GCCGCAGCCCCCTGCTCCGGAGGCTGGTGCAGGAAATAAGATATGTGGAACGGAGTTATGTATCAAAACCCACTTTGAAG\n\
    GAAGTGGTCATAGTAAGTGCTACAAGAACACCCATTGGATCTTTTTTAGGCAGCCTTTCCTTGCTGCCAGCCACTAAGCT\n\
    TGGTTCCATTGCAATTCAGGGAGCCATTGAAAAGGCAGGGATTCCAAAAGAAGAAGTGAAAGAAGCATACATGGGTAATG\n\
    TTCTACAAGGAGGTGAAGGACAAGCTCCTACAAGGCAGGCAGTATTGGGTGCAGGCTTACCTATTTCTACTCCATGTACC\n\
    ACCATAAACAAAGTTTGTGCTTCAGGAATGAAAGCCATCATGATGGCCTCTCAAAGTCTTATGTGTGGACATCAGGATGT\n\
    GATGGTGGCAGGTGGGATGGAGAGCATGTCCAATGTTCCATATGTAATGAACAGAGGATCAACACCATATGGTGGGGTAA\n\
    AGCTTGAAGATTTGATTGTAAAAGACGGGCTAACTGATGTCTACAATAAAATTCATATGGGCAGCTGTGCTGAGAATACA\n\
    GCAAAGAAGCTGAATATTGCACGAAATGAACAGGACGCTTATGCTATTAATTCTTATACCAGAAGTAAAGCAGCATGGGA\n\
    AGCTGGGAAATTTGGAAATGAAGTTATTCCTGTCACAGTTACAGTAAAAGGTCAACCAGATGTAGTGGTGAAAGAAGATG\n\
    AAGAATATAAACGTGTTGATTTTAGCAAAGTTCCAAAGCTGAAGACAGTTTTCCAGAAAGAAAATGGCACAGTAACAGCT\n\
    GCCAATGCCAGTACACTGAATGATGGAGCAGCTGCTCTGGTTCTCATGACGGCAGATGCAGCGAAGAGGCTCAATGTTAC\n\
    ACCACTGGCAAGAATAGTAGCATTTGCTGACGCTGCTGTAGAACCTATTGATTTTCCAATTGCTCCTGTATATGCTGCAT\n\
    CTATGGTTCTTAAAGATGTGGGATTGAAAAAAGAAGATATTGCAATGTGGGAAGTAAATGAAGCCTTTAGTCTGGTTGTA\n\
    CTAGCAAACATTAAAATGTTGGAGATTGATCCCCAAAAAGTGAATATCAATGGAGGAGCTGTTTCTCTGGGACATCCAAT\n\
    TGGGATGTCTGGAGCCAGGATTGTTGGTCATTTGACTCATGCCTTGAAGCAAGGAGAATACGGTCTTGCCAGTATTTGCA\n\
    ATGGAGGAGGAGGTGCTTCTGCCATGCTAATTCAGAAGCTGTAGACAACCTCTGCTATTTAAGGAGACAACCCTATGTGA\n\
    CCAGAAGGCCTGCTGTAATCAGTGTGACTACTGTGGGTCAGCTTATATTCAGATAAGCTGTTTCATTTTTTATTATTTTC\n\
    TATGTTAACTTTTAAAAATCAAAATGATGAAATCCCAAAACATTTTGAAATTAAAAATAAATTTCTTCTTCTGCTTTTTT\n\
    CTTGGTAACCTTGAAAA\n\
    >NM_000020.3 Homo sapiens activin A receptor like type 1 (ACVRL1), transcript variant 1, mRNA\n\
    CCCAGTCCCGGGAGGCTGCCGCGCCAGCTGCGCCGAGCGAGCCCCTCCCCGGCTCCAGCCCGGTCCGGGGCCGCGCCCGG\n\
    ACCCCAGCCCGCCGTCCAGCGCTGGCGGTGCAACTGCGGCCGCGCGGTGGAGGGGAGGTGGCCCCGGTCCGCCGAAGGCT\n\
    AGCGCCCCGCCACCCGCAGAGCGGGCCCAGAGGGACCATGACCTTGGGCTCCCCCAGGAAAGGCCTTCTGATGCTGCTGA\n\
    TGGCCTTGGTGACCCAGGGAGACCCTGTGAAGCCGTCTCGGGGCCCGCTGGTGACCTGCACGTGTGAGAGCCCACATTGC\n\
    AAGGGGCCTACCTGCCGGGGGGCCTGGTGCACAGTAGTGCTGGTGCGGGAGGAGGGGAGGCACCCCCAGGAACATCGGGG\n\
    CTGCGGGAACTTGCACAGGGAGCTCTGCAGGGGGCGCCCCACCGAGTTCGTCAACCACTACTGCTGCGACAGCCACCTCT\n\
    GCAACCACAACGTGTCCCTGGTGCTGGAGGCCACCCAACCTCCTTCGGAGCAGCCGGGAACAGATGGCCAGCTGGCCCTG\n\
    ATCCTGGGCCCCGTGCTGGCCTTGCTGGCCCTGGTGGCCCTGGGTGTCCTGGGCCTGTGGCATGTCCGACGGAGGCAGGA"; // 64 lines

// Fourth block of the Genome File
const GENOME_BLOCK_4: &str =
	"GAAGCAGCGTGGCCTGCACAGCGAGCTGGGAGAGTCCAGTCTCATCCTGAAAGCATCTGAGCAGGGCGACAGCATGTTGG\n\
    GGGACCTCCTGGACAGTGACTGCACCACAGGGAGTGGCTCAGGGCTCCCCTTCCTGGTGCAGAGGACAGTGGCACGGCAG\n\
    GTTGCCTTGGTGGAGTGTGTGGGAAAAGGCCGCTATGGCGAAGTGTGGCGGGGCTTGTGGCACGGTGAGAGTGTGGCCGT\n\
    CAAGATCTTCTCCTCGAGGGATGAACAGTCCTGGTTCCGGGAGACTGAGATCTATAACACAGTGTTGCTCAGACACGACA\n\
    ACATCCTAGGCTTCATCGCCTCAGACATGACCTCCCGCAACTCGAGCACGCAGCTGTGGCTCATCACGCACTACCACGAG\n\
    CACGGCTCCCTCTACGACTTTCTGCAGAGACAGACGCTGGAGCCCCATCTGGCTCTGAGGCTAGCTGTGTCCGCGGCATG\n\
    CGGCCTGGCGCACCTGCACGTGGAGATCTTCGGTACACAGGGCAAACCAGCCATTGCCCACCGCGACTTCAAGAGCCGCA\n\
    ATGTGCTGGTCAAGAGCAACCTGCAGTGTTGCATCGCCGACCTGGGCCTGGCTGTGATGCACTCACAGGGCAGCGATTAC\n\
    CTGGACATCGGCAACAACCCGAGAGTGGGCACCAAGCGGTACATGGCACCCGAGGTGCTGGACGAGCAGATCCGCACGGA\n\
    CTGCTTTGAGTCCTACAAGTGGACTGACATCTGGGCCTTTGGCCTGGTGCTGTGGGAGATTGCCCGCCGGACCATCGTGA\n\
    ATGGCATCGTGGAGGACTATAGACCACCCTTCTATGATGTGGTGCCCAATGACCCCAGCTTTGAGGACATGAAGAAGGTG\n\
    GTGTGTGTGGATCAGCAGACCCCCACCATCCCTAACCGGCTGGCTGCAGACCCGGTCCTCTCAGGCCTAGCTCAGATGAT\n\
    GCGGGAGTGCTGGTACCCAAACCCCTCTGCCCGACTCACCGCGCTGCGGATCAAGAAGACACTACAAAAAATTAGCAACA\n\
    GTCCAGAGAAGCCTAAAGTGATTCAATAGCCCAGGAGCACCTGATTCCTTTCTGCCTGCAGGGGGCTGGGGGGGTGGGGG\n\
    GCAGTGGATGGTGCCCTATCTGGGTAGAGGTAGTGTGAGTGTGGTGTGTGCTGGGGATGGGCAGCTGCGCCTGCCTGCTC\n\
    GGCCCCCAGCCCACCCAGCCAAAAATACAGCTGGGCTGAAACCTGATCCCCTGCTGTCTGGCCTGCTCAAAGCGGCAGGC\n\
    TCCCTGACGCCTGGCTCTCTCCCCACCCCTATGGCCAGCATGGTGCACCCCCTACCACTCCCGGGACAGGATGCAAAAGA\n\
    GGCTCCAGAGTCAGAGTGCCAAGCCAGGGAATCCCAGTCCCAGACTCAGAGCCCGGGCCTGCACTTTGCCCCCTGCCCTT\n\
    GATCAACCCCACTGCCCCACCAGAGCTGCCAGGGTGGCACAGGGCCCTGTCCAGCCCCTGGCACACACTTCCCTGCCAGG\n\
    CCTCAGCCTCTAGCATAAGCTCCAGAGAGCCAGGGCCCATCAGTTTCTCTCTGTGGATTTGTATCTCAGCTCCATGATGC\n\
    CTTGGGCTTTCTGTCTCCTCAACAAGAGTGCAGCTTGCTGAATGTCAGCTGCCTGAGAGAGCTGGGGCCTGACTTACTAG\n\
    GGCATTAAATCCTAAGAGGTCCTACTGAGGTGTGGCAGGATCACAGGCCAGTGGAAAAAGGGCAGGTCAGATGGGCAAGG\n\
    CCCAGGACTTTCAGATTAACTGAGAGGATATCGAGGCCAAGCATGGCAGGGGGAAGGTCAGTGGGTGTCAAGAGACCCAG\n\
    GTCTGACCCCGGATGTTTGCTCCATGTGACAAAAGCAGGCCTGTCTCAGGACCTTTTCTTTTCTTTTTTCCTTCTTTTTT\n\
    TTTTTGACACGGAGTTTCGCTCTTGTTGTCCAGGCTAGAGTGCAATGGCATGATCCCAGCTCACCGCAACGTCTACCTCC\n\
    CAGGTTCAAATCATTCTCTTGCCTCAGACTCCCGAGTAGCTGGGATTACAGGCACATGCCACCATGCCTGGCTAATTTTG\n\
    TATATTTAGTAGAAACAGGGTTTCACCATGCTGGCCATGCTGGTCTCGAACTCCTGACCTCAGGTGTTCCACCTACCTCA\n\
    GCCTCCCAAAGTGCTGGGGTTACAGGTGTGAGCCATCGCGCCTGGCCAGGACCTTTGTTTCTTATCTACATATTGGAAGA\n\
    TTTGGTCCTGATGTCCTTTGAGGCTTCTTTAGCTCTAGTTCTCTGACACTTCAGCCTATATCACAGCTAACTTCTTCAGT\n\
    CTCATCTATTCCTTATGCTCCAGCCCCTGGCAATTTGCCTCAAGATGGGGGTTTGAAAATAACTTTACCTGACTCAAGGA\n\
    GTGTCTGGAGCACCTCCTAGTCTAAGTCTGCAAGCTCCAGTTCTTGCCTAAAACCATGCCAGTGGCCACCCTTGGGCTCA\n\
    GACAGCTCTGGGCCTTTTGACCACAAGCCAGCCCCTCGCCCTCTCTGTGGCATAGTCTTCTCTGCCCCAGGACTGCAGGG\n\
    CGGCTTCCTCCAAGGCTTCCAAGGCTCAAAAGAAATTTGGCTCCATCCAAGAAGGCTCCAGCTCCCCTACTGGCCCCTGG\n\
    CTCAGGCCCACACCCCTGGCCAGGCCCAGAGAGTGTGTCTCAGGAGAATTCAATGGCTCTAGAGAGACACACAGAAAGTT\n\
    TGGCATTTGGAAATTTCAAGGATGTATGTATGCTCACGTATGGAGCAGGTTGTCCTGGTCCCTGGGTGCAGGGAAGTGGG\n\
    CTGCAGGGAAGTGGATTGGAGGGGAGCTTGAGGAATATAAGGAGCGGGGGTGGAGACTCAGGCTATGGACAAGGACAGCC\n\
    CCAAGGTTGGGAAGACCTGGCCTTAGTCGTCCTCAGCCTAGGGGCAGGGCAGTGAAGAAAGCTCTCCCCGCTCCTGCTGT\n\
    AATGACCCAGAGTAGCCTCCCCAGGCCGGCATCTTATGTGTGTCTTCCACCATCCTCATGGTGGCACTTTTCTAGGCCTG\n\
    TCTCCCAGCATTGTGCAAGGCTCGGAAGAGAACCAGGAAGTGAAACTGGGTGAAAACAGAAAGCTCAATGGATGGGCTAG\n\
    GTTCCCAGATCATTAGGGCAGAGTTTGCACGTCCTCTGGTCACTGGAATCCACCCAGCCCACGAATCATCTCCCTCTTGA\n\
    AGGATTTTATTTCTACTGGGTTTTGGAACAAACTCCTGCTGAGACCCCACAGCCAGAAACTGAAAGCAGCAGCTCCCCAA\n\
    AGCCTGGAAAATCCCTAAGAGAAGGCCTGGGGCAGGAAGTGGAGTGACAGGGGACAGGTAGAGAGAAGGGGGCCCAATGG\n\
    CCAGGGAGTGAAGGAGGTGGCGTTGCTGAGAGCAGTCTGCACATGCTTCTGTCTGAGTGCAGGAAGGTGTTCCAGGGTCG\n\
    AAATTACACTTCTCGTACCTGGAGACGCTGTTTGTGGGAGCACTGGGCTCATGCCTGGCACACAATAGGTCTGCAATAAA\n\
    CCATGGTTAAATCCTGA\n\
    >NM_000021.4 Homo sapiens presenilin 1 (PSEN1), transcript variant 1, mRNA\n\
    GGAAACAAAACAGCGGCTGGTCTGGAAGGAACCTGAGCTACGAGCCGCGGCGGCAGCGGGGCGGCGGGGAAGCGTATACC\n\
    TAATCTGGGAGCCTGCAAGTGACAACAGCCTTTGCGGTCCTTAGACAGCTTGGCCTGGAGGAGAACACATGAAAGAAAGA\n\
    ACCTCAAGAGGCTTTGTTTTCTGTGAAACAGTATTTCTATACAGTTGCTCCAATGACAGAGTTACCTGCACCGTTGTCCT\n\
    ACTTCCAGAATGCACAGATGTCTGAGGACAACCACCTGAGCAATACTGTACGTAGCCAGAATGACAATAGAGAACGGCAG\n\
    GAGCACAACGACAGACGGAGCCTTGGCCACCCTGAGCCATTATCTAATGGACGACCCCAGGGTAACTCCCGGCAGGTGGT\n\
    GGAGCAAGATGAGGAAGAAGATGAGGAGCTGACATTGAAATATGGCGCCAAGCATGTGATCATGCTCTTTGTCCCTGTGA\n\
    CTCTCTGCATGGTGGTGGTCGTGGCTACCATTAAGTCAGTCAGCTTTTATACCCGGAAGGATGGGCAGCTAATCTATACC\n\
    CCATTCACAGAAGATACCGAGACTGTGGGCCAGAGAGCCCTGCACTCAATTCTGAATGCTGCCATCATGATCAGTGTCAT\n\
    TGTTGTCATGACTATCCTCCTGGTGGTTCTGTATAAATACAGGTGCTATAAGGTCATCCATGCCTGGCTTATTATATCAT\n\
    CTCTATTGTTGCTGTTCTTTTTTTCATTCATTTACTTGGGGGAAGTGTTTAAAACCTATAACGTTGCTGTGGACTACATT\n\
    ACTGTTGCACTCCTGATCTGGAATTTTGGTGTGGTGGGAATGATTTCCATTCACTGGAAAGGTCCACTTCGACTCCAGCA\n\
    GGCATATCTCATTATGATTAGTGCCCTCATGGCCCTGGTGTTTATCAAGTACCTCCCTGAATGGACTGCGTGGCTCATCT\n\
    TGGCTGTGATTTCAGTATATGATTTAGTGGCTGTTTTGTGTCCGAAAGGTCCACTTCGTATGCTGGTTGAAACAGCTCAG\n\
    GAGAGAAATGAAACGCTTTTTCCAGCTCTCATTTACTCCTCAACAATGGTGTGGTTGGTGAATATGGCAGAAGGAGACCC\n\
    GGAAGCTCAAAGGAGAGTATCCAAAAATTCCAAGTATAATGCAGAAAGCACAGAAAGGGAGTCACAAGACACTGTTGCAG\n\
    AGAATGATGATGGCGGGTTCAGTGAGGAATGGGAAGCCCAGAGGGACAGTCATCTAGGGCCTCATCGCTCTACACCTGAG\n\
    TCACGAGCTGCTGTCCAGGAACTTTCCAGCAGTATCCTCGCTGGTGAAGACCCAGAGGAAAGGGGAGTAAAACTTGGATT\n\
    GGGAGATTTCATTTTCTACAGTGTTCTGGTTGGTAAAGCCTCAGCAACAGCCAGTGGAGACTGGAACACAACCATAGCCT"; // 64 lines

// Fifth block of the Genome File
const GENOME_BLOCK_5: &str =
	"GTTTCGTAGCCATATTAATTGGTTTGTGCCTTACATTATTACTCCTTGCCATTTTCAAGAAAGCATTGCCAGCTCTTCCA\n\
    ATCTCCATCACCTTTGGGCTTGTTTTCTACTTTGCCACAGATTATCTTGTACAGCCTTTTATGGACCAATTAGCATTCCA\n\
    TCAATTTTATATCTAGCATATTTGCGGTTAGAATCCCATGGATGTTTCTTCTTTGACTATAACAAAATCTGGGGAGGACA\n\
    AAGGTGATTTTCCTGTGTCCACATCTAACAAAGTCAAGATTCCCGGCTGGACTTTTGCAGCTTCCTTCCAAGTCTTCCTG\n\
    ACCACCTTGCACTATTGGACTTTGGAAGGAGGTGCCTATAGAAAACGATTTTGAACATACTTCATCGCAGTGGACTGTGT\n\
    CCCTCGGTGCAGAAACTACCAGATTTGAGGGACGAGGTCAAGGAGATATGATAGGCCCGGAAGTTGCTGTGCCCCATCAG\n\
    CAGCTTGACGCGTGGTCACAGGACGATTTCACTGACACTGCGAACTCTCAGGACTACCGTTACCAAGAGGTTAGGTGAAG\n\
    TGGTTTAAACCAAACGGAACTCTTCATCTTAAACTACACGTTGAAAATCAACCCAATAATTCTGTATTAACTGAATTCTG\n\
    AACTTTTCAGGAGGTACTGTGAGGAAGAGCAGGCACCAGCAGCAGAATGGGGAATGGAGAGGTGGGCAGGGGTTCCAGCT\n\
    TCCCTTTGATTTTTTGCTGCAGACTCATCCTTTTTAAATGAGACTTGTTTTCCCCTCTCTTTGAGTCAAGTCAAATATGT\n\
    AGATTGCCTTTGGCAATTCTTCTTCTCAAGCACTGACACTCATTACCGTCTGTGATTGCCATTTCTTCCCAAGGCCAGTC\n\
    TGAACCTGAGGTTGCTTTATCCTAAAAGTTTTAACCTCAGGTTCCAAATTCAGTAAATTTTGGAAACAGTACAGCTATTT\n\
    CTCATCAATTCTCTATCATGTTGAAGTCAAATTTGGATTTTCCACCAAATTCTGAATTTGTAGACATACTTGTACGCTCA\n\
    CTTGCCCCAGATGCCTCCTCTGTCCTCATTCTTCTCTCCCACACAAGCAGTCTTTTTCTACAGCCAGTAAGGCAGCTCTG\n\
    TCGTGGTAGCAGATGGTCCCATTATTCTAGGGTCTTACTCTTTGTATGATGAAAAGAATGTGTTATGAATCGGTGCTGTC\n\
    AGCCCTGCTGTCAGACCTTCTTCCACAGCAAATGAGATGTATGCCCAAAGACGGTAGAATTAAAGAAGAGTAAAATGGCT\n\
    GTTGAAGCACTTTCTGTCCTGGTATTTTGTTTTTGCTTTTGCCACACAGTAGCTCAGAATTTGAACAAATAGCCAAAAGC\n\
    TGGTGGTTGATGAATTATGAACTAGTTGTATCAACACAAAGCAAGAGTTGGGGAAAGCCATATTTAACTTGGTGAGCTGT\n\
    GGGAGAACCTGGTGGCAGAAGGAGAACCAACTGCCAAGGGGAAAGAGAAGGGGCCTCCAGCAGCGAAGGGGATACAGTGA\n\
    GCTAATGATGTCAAGGAGGAGTTTCAGGTTATTCTCGTCAGCTCCACAAATGGGTGCTTTGTGGTCTCTGCCCGCGTTAC\n\
    CTTTCCTCTCAATGTACCTTTGTGTGAACTGGGCAGTGGAGGTGCCTGCTGCAGTTACCATGGAGTTCAGGCTCTGGGCA\n\
    GCTCAGTCAGGCAAAACACACAAACAGCCATCAGCCTGTGTGGGCTCAGGGCACCTCTGGACAAAGGCTTGTGGGGCATA\n\
    ACCTTCTTTACCACAGAGAGCCCTTAGCTATGCTGATCAGACCGTAAGCGTTTATGAGAAACTTAGTTTCCTCCTGTGGC\n\
    TGAGGAGGGGCCAGCTTTTTCTTCTTTTGCCTGCTGTTTTCTCTCCCAATCTATGATATGATATGACCTGGTTTGGGGCT\n\
    GTCTTTGGTGTTTAGAATATTTGTTTTCTGTCCCAGGATATTTCTTATAAGAACCTAACTTCAAGAGTAGTGTGCGAGTA\n\
    CTGATCTGAATTTAAATTAAAATTGGCTTATATTAGGCAGTCACAGACAGGAAAAATAAGAGCTATGCAAAGAAAGGGGG\n\
    ATTTAAAGTAGTAGGTTCTATCATCTCAATTCATTTTTTTCCATGAAATCCCTTCTTCCAAGATTCATTCCCTCTCTCAG\n\
    ACATGTGCTAGCATGGGTATTATCATTGAGAAAGCACAGCTACAGCAAAGCCACCTGAATAGCAATTTGTGATTGGAAGC\n\
    ATTCTTGAGGGATCCCTAATCTAGAGTAATTTATTTGTGTAAGGATCCCAAATGTGTTGCACCTTTCATGATACATTTCT\n\
    TCTCTGAAGAGGGTACGTGGGGTGTGTGTATTTAAATCCATCCTATGTATTACTGATTGTCCTGTGTAGAAAGATGGCAA\n\
    TTATTCTGTCTCTTTCTCCAAGTTTGAGCCACATCTCAGCCACATTGTTAGACAGTGTACAGAGAACCTATCTTTCCTTT\n\
    TTTTTTTTTTAAAGGACAGGATTTTGCTGTGTTGCCCAGGCTAGACTTGAACTCCTGGGCTCAAGTAATCCACCTCAGCC\n\
    TGAGTAGCTGAGACTACAGCCCATCTTATTTCTTTAAATCATTCATCTCAGGCAGAGAACTTTTCCCTCAAACATTCTTT\n\
    TTAGAATTAGTTCAGTCATTCCTAAAACATCCAAATGCTAGTCTTCCACCATGAAAAATAGATTGTCACTGGAAAGAACA\n\
    GTAGCAATTTCCATAAGGATGTGCCTTCACTCACACGGGACAGGCGGTGGTTATAGAGTCGGGCAAAACCAGCAGTAGAG\n\
    TATGACCAGCCAAGCCAATCTGCTTAATAAAAAGATGGAAGACAGTAAGGAAGGAAAGTAGCCACTAAGAGTCTGAGTCT\n\
    GACTGGGCTACAGAATAAAGGGTATTTATGGACAGAATGTCATTACATGCCTATGGGAATACCAATCATATTTGGAAGAT\n\
    TTGCAGATTTTTTTTCAGAGAGGAAAGACTCACCTTCCTGTTTTTGGTTCTCAGTAGGTTCGTGTGTGTTCCTAGAATCA\n\
    CAGCTCTGACTCCAAATGACTCAATTTCTCAATTAGAAAAAGTAGAAGCTTTCTAAGCAACTTGGAAGAAAACAGTCATA\n\
    AGTAAGCAATTTGTTGATTTTACTACAGAAGCAACAACTGAAGAGGCAGTGTTTTTACTTTCAGACTCCGGGATTCCCAT\n\
    TCTGTAGTCTCTCTGCTTTTAAAAACCCTCCTTTTGCAATAGATGCCCAAACAGATGATGTTTATTACTTGTTATTTACG\n\
    TGGCCTCAGACAGTGTATGTATTCTCGATATAACTTGTAGAGTGTGAAATATAAGTTTAACTACCAAATAAGGTCTCCCA\n\
    GGGTTAGATGACTGCGGGAAGCCTTTGATCCCAACCCCCAAGGCTTTGTATATTTGATCATTTGTGATCTAACCCTGGAA\n\
    GAAAAAGAGCTCAGAAACCACTATGAAAAAATTTGTTCAGTGTTTTCTGTGTTCCCGTAGGTTCTGGAGTCTGAGGATGC\n\
    AAAGATGAATAAGATAAATTCTCAGAATGTAGTTATAATCTCTTGTTTTCTGGTATATGCCATCTTTCTTTAACTTCTCT\n\
    AAAATATTGGGTATTTGTCAAATAACCACTTTTAACAGTTACCATTACTGAGGGCTTATACATTGGTGTTATAAAAGTGA\n\
    CTTGATTCAGAAATCAATCCATTCAGTAAAGTACTCCTTCTCTAAATTTGCTGTTATGTCTATAAGGAACAGTTTGACCT\n\
    GCCCTTCTCCTCACCTCCTCACCTGCCTTCCAACATTGAATTTGGAAGGAGACGTGAAAATTGGACATTTGGTTTTGCCC\n\
    TTGGGCTGGAAACTATCATATAATCATAAGTTTGAGCCTAGAAGTGATCCTTGTGATCTTCTCACCTCTTTAAATTCCCA\n\
    CAACACAAGAGATTAAAAACAGAGGTTTCAGCTCTTCATAGTGCGTTGTGAAATGGCTGGCCAGAGTGTACCAACAAAGC\n\
    TGTCATCGGGCTCACAGCTCAGAGACATCTGCATGTGATCATCTGCATAGTCCTCTCCTCTAACGGGAAACACCTCAGAT\n\
    TTGCATATAAAAAAGCACCCTGGTGCTGAAATGAACCCCTTTCTTGAACATCAAAGCTGTCTCCCACAGCCTTGGGCAGC\n\
    AGGGTGCCTCTTAGTGGATGTGCTGGGTCCACCCTGAGCCCTGACATGTGGTGGCAGCATTGCCAGTTGGTCTGTGTGTC\n\
    TGTGTAGCAGGGACGATTTCCCAGAAAGCAATTTTCCTTTTGAAATACGTAATTGTTGAGACTAGGCAGTTTCAAAGTCA\n\
    GCTGCATATAGTAGCAAGTACAGGACTGTCTTGTTTTTGGTGTCCTTGGAGGTGCTGGGGTGAGGGTTTCAGTGGGATCA\n\
    TTTACTCTCACATGTTGTCTGCCTTCTGCTTCTGTGGACACTGCTTTGTACTTAATTCAGACAGACTGTGAATACACCTT\n\
    TTTTATAAATACCTTTCAAATTCTTGGTAAGATATAATTTTGATAGCTGATTGCAGATTTTCTGTATTTGTCAGATTAAT\n\
    AAAGACTGCATGAATCCA\n\
    >NM_000022.4 Homo sapiens adenosine deaminase (ADA), transcript variant 1, mRNA\n\
    GCTGGCCCCAGGGAAAGCCGAGCGGCCACCGAGCCGGCAGAGACCCACCGAGCGGCGGCGGAGGGAGCAGCGCCGGGGCG\n\
    CACGAGGGCACCATGGCCCAGACGCCCGCCTTCGACAAGCCCAAAGTGGAACTGCATGTCCACCTAGACGGATCCATCAA\n\
    GCCTGAAACCATCTTATACTATGGCAGGAGGAGAGGGATCGCCCTCCCAGCTAACACAGCAGAGGGGCTGCTGAACGTCA\n\
    TTGGCATGGACAAGCCGCTCACCCTTCCAGACTTCCTGGCCAAGTTTGACTACTACATGCCTGCTATCGCGGGCTGCCGG\n\
    GAGGCTATCAAAAGGATCGCCTATGAGTTTGTAGAGATGAAGGCCAAAGAGGGCGTGGTGTATGTGGAGGTGCGGTACAG\n\
    TCCGCACCTGCTGGCCA"; // 65 lines

/*
============================================
			SMALL FILE CONSTANTS
============================================
*/

// The only block of the SMALL_FILE
const SMALL_FILE_BLOCK: &str = "#include<stdio.h>\n\
    \n\
    void test_func() {\n\
    \tprintf(\"Testing the Blocks construction 🥹\\n\");\n\
    }\n\
    \n\
    int main() {\n\
	\tprintf(\"Hopefully it works 🥹🇺🇸🇳🇴\\n\");\n\
	\ttest_func();\n\
    \n\
	\treturn 0;\n\
    }\n\
    ";

/*
===========================================
			HIGHLIGHT CONSTANTS
===========================================
*/

// The results of deleting a selection on a single line
const SINGLE_LINE_SELECTION_DELETION: &str = "123456789🥹\n\
    abcdefghi\n\
    ^&*(\n\
    jklmnopqr\n\
    987654321\n\
    +_)=-\\🥹,./";

// The results of deleting a selection over multiple lines
const MULTI_LINE_SELECTION_DELETION: &str = "123456789🥹\n\
    abc4321\n\
    +_)=-\\🥹,./";

/*
			KEY FUNCTIONS CONSTANTS
*/

// The modification that the saved debug file should contain (for SMALL_FILE)
const MODIFIED_SMALL_SAVE_FILE: &str = "#include<stdio.h>\n\
    \n\
    void test_func() {	printf(\"Testing the Blocks construction 🥹\\n\");\n\
    }\n\
    int main() {\n\
	\tprintf(\"Hopefully it works 🥹🇺🇸🇳🇴\\n\");\n\
	\ttest_func();\n\
    \n\
	\treturn 0;\n\
    }\n\
    ";

// The modification that the saved debug file should contain (for GENOME_FILE)
const MODIFIED_LARGE_SAVE_FILE: &str =
	">NM_000014.6 Homo sapiens alpha-2-macroglobulin (A2M), transcript variant 1, mRNA\n\
    >NM_000022.4 Homo sapiens adenosine deaminase (ADA), transcript variant 1, mRNA\n\
    GCTGGCCCCAGGGAAAGCCGAGCGGCCACCGAGCCGGCAGAGACCCACCGAGCGGCGGCGGAGGGAGCAGCGCCGGGGCG\n\
    CACGAGGGCACCATGGCCCAGACGCCCGCCTTCGACAAGCCCAAAGTGGAACTGCATGTCCACCTAGACGGATCCATCAA\n\
    GCCTGAAACCATCTTATACTATGGCAGGAGGAGAGGGATCGCCCTCCCAGCTAACACAGCAGAGGGGCTGCTGAACGTCA\n\
    TTGGCATGGACAAGCCGCTCACCCTTCCAGACTTCCTGGCCAAGTTTGACTACTACATGCCTGCTATCGCGGGCTGCCGG\n\
    GAGGCTATCAAAAGGATCGCCTATGAGTTTGTAGAGATGAAGGCCAAAGAGGGCGTGGTGTATGTGGAGGTGCGGTACAG\n\
    TCCGCACCTGCTGGCCA";
