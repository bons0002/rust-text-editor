use crate::editor::EditorSpace;
use key_functions::{down_arrow, up_arrow};
use rayon::iter::{IntoParallelIterator, ParallelExtend};

use super::*;

/* Tests for creating, loading, and unloading
blocks from different files. */
mod blocks_tests;
/* Tests for highlighting text and deleting
said highlight. */
mod selection_tests;

/*
==========================================
            FILENAME CONSTANTS
==========================================
*/

// Small file with very little text
const SMALL_FILE: &str = "../editor/test_files/small_text.txt";

// Large file of part of the human genome
const GENOME_FILE: &str = "../editor/test_files/GRCh38_50_rna.fna";

// File for highlighting tests
const HIGHLIGHT_FILE: &str = "../editor/test_files/highlight.txt";

// The result of the multi_deletion_test in selection_tests
const MULTI_DELETION: &str = "../editor/test_files/results/multi_deletion_test_result.txt";

/*
========================================
            BLOCKS CONSTANTS
========================================
*/

// The first block of the genome file
const FIRST_BLOCK_GENOME: &str =
	">NM_000014.6 Homo sapiens alpha-2-macroglobulin (A2M), transcript variant 1, mRNA\
    GGGACCAGATGGATTGTAGGGAGTAGGGTACAATACAGTCTGTTCTCCTCCAGCTCCTTCTTTCTGCAACATGGGGAAGA\
    ACAAACTCCTTCATCCAAGTCTGGTTCTTCTCCTCTTGGTCCTCCTGCCCACAGACGCCTCAGTCTCTGGAAAACCGCAG\
    TATATGGTTCTGGTCCCCTCCCTGCTCCACACTGAGACCACTGAGAAGGGCTGTGTCCTTCTGAGCTACCTGAATGAGAC\
    AGTGACTGTAAGTGCTTCCTTGGAGTCTGTCAGGGGAAACAGGAGCCTCTTCACTGACCTGGAGGCGGAGAATGACGTAC\
    TCCACTGTGTCGCCTTCGCTGTCCCAAAGTCTTCATCCAATGAGGAGGTAATGTTCCTCACTGTCCAAGTGAAAGGACCA\
    ACCCAAGAATTTAAGAAGCGGACCACAGTGATGGTTAAGAACGAGGACAGTCTGGTCTTTGTCCAGACAGACAAATCAAT\
    CTACAAACCAGGGCAGACAGTGAAATTTCGTGTTGTCTCCATGGATGAAAACTTTCACCCCCTGAATGAGTTGATTCCAC\
    TAGTATACATTCAGGATCCCAAAGGAAATCGCATCGCACAATGGCAGAGTTTCCAGTTAGAGGGTGGCCTCAAGCAATTT\
    TCTTTTCCCCTCTCATCAGAGCCCTTCCAGGGCTCCTACAAGGTGGTGGTACAGAAGAAATCAGGTGGAAGGACAGAGCA\
    CCCTTTCACCGTGGAGGAATTTGTTCTTCCCAAGTTTGAAGTACAAGTAACAGTGCCAAAGATAATCACCATCTTGGAAG\
    AAGAGATGAATGTATCAGTGTGTGGCCTATACACATATGGGAAGCCTGTCCCTGGACATGTGACTGTGAGCATTTGCAGA\
    AAGTATAGTGACGCTTCCGACTGCCACGGTGAAGATTCACAGGCTTTCTGTGAGAAATTCAGTGGACAGCTAAACAGCCA\
    TGGCTGCTTCTATCAGCAAGTAAAAACCAAGGTCTTCCAGCTGAAGAGGAAGGAGTATGAAATGAAACTTCACACTGAGG\
    CCCAGATCCAAGAAGAAGGAACAGTGGTGGAATTGACTGGAAGGCAGTCCAGTGAAATCACAAGAACCATAACCAAACTC\
    TCATTTGTGAAAGTGGACTCACACTTTCGACAGGGAATTCCCTTCTTTGGGCAGGTGCGCCTAGTAGATGGGAAAGGCGT\
    CCCTATACCAAATAAAGTCATATTCATCAGAGGAAATGAAGCAAACTATTACTCCAATGCTACCACGGATGAGCATGGCC\
    TTGTACAGTTCTCTATCAACACCACCAATGTTATGGGTACCTCTCTTACTGTTAGGGTCAATTACAAGGATCGTAGTCCC\
    TGTTACGGCTACCAGTGGGTGTCAGAAGAACACGAAGAGGCACATCACACTGCTTATCTTGTGTTCTCCCCAAGCAAGAG\
    CTTTGTCCACCTTGAGCCCATGTCTCATGAACTACCCTGTGGCCATACTCAGACAGTCCAGGCACATTATATTCTGAATG\
    GAGGCACCCTGCTGGGGCTGAAGAAGCTCTCCTTCTATTATCTGATAATGGCAAAGGGAGGCATTGTCCGAACTGGGACT\
    CATGGACTGCTTGTGAAGCAGGAAGACATGAAGGGCCATTTTTCCATCTCAATCCCTGTGAAGTCAGACATTGCTCCTGT\
    CGCTCGGTTGCTCATCTATGCTGTTTTACCTACCGGGGACGTGATTGGGGATTCTGCAAAATATGATGTTGAAAATTGTC\
    TGGCCAACAAGGTGGATTTGAGCTTCAGCCCATCACAAAGTCTCCCAGCCTCACACGCCCACCTGCGAGTCACAGCGGCT\
    CCTCAGTCCGTCTGCGCCCTCCGTGCTGTGGACCAAAGCGTGCTGCTCATGAAGCCTGATGCTGAGCTCTCGGCGTCCTC\
    GGTTTACAACCTGCTACCAGAAAAGGACCTCACTGGCTTCCCTGGGCCTTTGAATGACCAGGACAATGAAGACTGCATCA\
    ATCGTCATAATGTCTATATTAATGGAATCACATATACTCCAGTATCAAGTACAAATGAAAAGGATATGTACAGCTTCCTA\
    GAGGACATGGGCTTAAAGGCATTCACCAACTCAAAGATTCGTAAACCCAAAATGTGTCCACAGCTTCAACAGTATGAAAT\
    GCATGGACCTGAAGGTCTACGTGTAGGTTTTTATGAGTCAGATGTAATGGGAAGAGGCCATGCACGCCTGGTGCATGTTG\
    AAGAGCCTCACACGGAGACCGTACGAAAGTACTTCCCTGAGACATGGATCTGGGATTTGGTGGTGGTAAACTCAGCAGGT\
    GTGGCTGAGGTAGGAGTAACAGTCCCTGACACCATCACCGAGTGGAAGGCAGGGGCCTTCTGCCTGTCTGAAGATGCTGG\
    ACTTGGTATCTCTTCCACTGCCTCTCTCCGAGCCTTCCAGCCCTTCTTTGTGGAGCTCACAATGCCTTACTCTGTGATTC\
    GTGGAGAGGCCTTCACACTCAAGGCCACGGTCCTAAACTACCTTCCCAAATGCATCCGGGTCAGTGTGCAGCTGGAAGCC\
    TCTCCCGCCTTCCTAGCTGTCCCAGTGGAGAAGGAACAAGCGCCTCACTGCATCTGTGCAAACGGGCGGCAAACTGTGTC\
    CTGGGCAGTAACCCCAAAGTCATTAGGAAATGTGAATTTCACTGTGAGCGCAGAGGCACTAGAGTCTCAAGAGCTGTGTG\
    GGACTGAGGTGCCTTCAGTTCCTGAACACGGAAGGAAAGACACAGTCATCAAGCCTCTGTTGGTTGAACCTGAAGGACTA\
    GAGAAGGAAACAACATTCAACTCCCTACTTTGTCCATCAGGTGGTGAGGTTTCTGAAGAATTATCCCTGAAACTGCCACC\
    AAATGTGGTAGAAGAATCTGCCCGAGCTTCTGTCTCAGTTTTGGGAGACATATTAGGCTCTGCCATGCAAAACACACAAA\
    ATCTTCTCCAGATGCCCTATGGCTGTGGAGAGCAGAATATGGTCCTCTTTGCTCCTAACATCTATGTACTGGATTATCTA\
    AATGAAACACAGCAGCTTACTCCAGAGATCAAGTCCAAGGCCATTGGCTATCTCAACACTGGTTACCAGAGACAGTTGAA\
    CTACAAACACTATGATGGCTCCTACAGCACCTTTGGGGAGCGATATGGCAGGAACCAGGGCAACACCTGGCTCACAGCCT\
    TTGTTCTGAAGACTTTTGCCCAAGCTCGAGCCTACATCTTCATCGATGAAGCACACATTACCCAAGCCCTCATATGGCTC\
    TCCCAGAGGCAGAAGGACAATGGCTGTTTCAGGAGCTCTGGGTCACTGCTCAACAATGCCATAAAGGGAGGAGTAGAAGA\
    TGAAGTGACCCTCTCCGCCTATATCACCATCGCCCTTCTGGAGATTCCTCTCACAGTCACTCACCCTGTTGTCCGCAATG\
    CCCTGTTTTGCCTGGAGTCAGCCTGGAAGACAGCACAAGAAGGGGACCATGGCAGCCATGTATATACCAAAGCACTGCTG\
    GCCTATGCTTTTGCCCTGGCAGGTAACCAGGACAAGAGGAAGGAAGTACTCAAGTCACTTAATGAGGAAGCTGTGAAGAA\
    AGACAACTCTGTCCATTGGGAGCGCCCTCAGAAACCCAAGGCACCAGTGGGGCATTTTTACGAACCCCAGGCTCCCTCTG\
    CTGAGGTGGAGATGACATCCTATGTGCTCCTCGCTTATCTCACGGCCCAGCCAGCCCCAACCTCGGAGGACCTGACCTCT\
    GCAACCAACATCGTGAAGTGGATCACGAAGCAGCAGAATGCCCAGGGCGGTTTCTCCTCCACCCAGGACACAGTGGTGGC\
    TCTCCATGCTCTGTCCAAATATGGAGCAGCCACATTTACCAGGACTGGGAAGGCTGCACAGGTGACTATCCAGTCTTCAG\
    GGACATTTTCCAGCAAATTCCAAGTGGACAACAACAACCGCCTGTTACTGCAGCAGGTCTCATTGCCAGAGCTGCCTGGG\
    GAATACAGCATGAAAGTGACAGGAGAAGGATGTGTCTACCTCCAGACATCCTTGAAATACAATATTCTCCCAGAAAAGGA\
    AGAGTTCCCCTTTGCTTTAGGAGTGCAGACTCTGCCTCAAACTTGTGATGAACCCAAAGCCCACACCAGCTTCCAAATCT\
    CCCTAAGTGTCAGTTACACAGGGAGCCGCTCTGCCTCCAACATGGCGATCGTTGATGTGAAGATGGTCTCTGGCTTCATT\
    CCCCTGAAGCCAACAGTGAAAATGCTTGAAAGATCTAACCATGTGAGCCGGACAGAAGTCAGCAGCAACCATGTCTTGAT\
    TTACCTTGATAAGGTGTCAAATCAGACACTGAGCTTGTTCTTCACGGTTCTGCAAGATGTCCCAGTAAGAGATCTGAAAC\
    CAGCCATAGTGAAAGTCTATGATTACTACGAGACGGATGAGTTTGCAATTGCTGAGTACAATGCTCCTTGCAGCAAAGAT\
    CTTGGAAATGCTTGAAGACCACAAGGCTGAAAAGTGCTTTGCTGGAGTCCTGTTCTCAGAGCTCCACAGAAGACACGTGT\
    TTTTGTATCTTTAAAGACTTGATGAATAAACACTTTTTCTGGTCAATGTC\
    >NM_000015.3 Homo sapiens N-acetyltransferase 2 (NAT2), mRNA\
    ACTTTATTACAGACCTTGGAAGCAAGAGGATTGCATTCAGCCTAGTTCCTGGTTGCTGGCCAAAGGGATCATGGACATTG\
    AAGCATATTTTGAAAGAATTGGCTATAAGAACTCTAGGAACAAATTGGACTTGGAAACATTAACTGACATTCTTGAGCAC\
    CAGATCCGGGCTGTTCCCTTTGAGAACCTTAACATGCATTGTGGGCAAGCCATGGAGTTGGGCTTAGAGGCTATTTTTGA\
    TCACATTGTAAGAAGAAACCGGGGTGGGTGGTGTCTCCAGGTCAATCAACTTCTGTACTGGGCTCT"; // 64 lines (last line incomplete = 63)

// Second block of the genome file
const SECOND_BLOCK_GENOME: &str = 
    "GACCACAATCGGTT\
    TTCAGACCACAATGTTAGGAGGGTATTTTTACATCCCTCCAGTTAACAAATACAGCACTGGCATGGTTCACCTTCTCCTG\
    CAGGTGACCATTGACGGCAGGAATTACATTGTCGATGCTGGGTCTGGAAGCTCCTCCCAGATGTGGCAGCCTCTAGAATT\
    AATTTCTGGGAAGGATCAGCCTCAGGTGCCTTGCATTTTCTGCTTGACAGAAGAGAGAGGAATCTGGTACCTGGACCAAA\
    TCAGGAGAGAGCAGTATATTACAAACAAAGAATTTCTTAATTCTCATCTCCTGCCAAAGAAGAAACACCAAAAAATATAC\
    TTATTTACGCTTGAACCTCGAACAATTGAAGATTTTGAGTCTATGAATACATACCTGCAGACGTCTCCAACATCTTCATT\
    TATAACCACATCATTTTGTTCCTTGCAGACCCCAGAAGGGGTTTACTGTTTGGTGGGCTTCATCCTCACCTATAGAAAAT\
    TCAATTATAAAGACAATACAGATCTGGTCGAGTTTAAAACTCTCACTGAGGAAGAGGTTGAAGAAGTGCTGAGAAATATA\
    TTTAAGATTTCCTTGGGGAGAAATCTCGTGCCCAAACCTGGTGATGGATCCCTTACTATTTAGAATAAGGAACAAAATAA\
    ACCCTTGTGTATGTATCACCCAACTCACTAATTATCAACTTATGTGCTATCAGATATCCTCTCTACCCTCACGTTATTTT\
    GAAGAAAATCCTAAACATCAAATACTTTCATCCATAAAAATGTCAGCATTTATTAAAAAACAATAACTTTTTAAAGAAAC\
    ATAAGGACACATTTTCAAATTAATAAAAATAAAGGCATTTTAAGGATGGCCTGTGATTATCTTGGGAAGCAGAGTGATTC\
    ATGCTAGAAAACATTTAATATTGATTTATTGTTGAATTCATAGTAAATTTTTACTGGTAAATGAATAAAGAATATTGTGG\
    AAAAA\
    >NM_000016.6 Homo sapiens acyl-CoA dehydrogenase medium chain (ACADM), transcript variant 1, mRNA; nuclear gene for mitochondrial product\
    AGAGGAGTCCCGCGTTCGGGGAGTATGTCAAGGCCGTGACCCGTGTATTATTGTCCGAGTGGCCGGAACGGGAGCCAACA\
    TGGCAGCGGGGTTCGGGCGATGCTGCAGGGTCCTGAGAAGTATTTCTCGTTTTCATTGGAGATCACAGCATACAAAAGCC\
    AATCGACAACGTGAACCAGGATTAGGATTTAGTTTTGAGTTCACCGAACAGCAGAAAGAATTTCAAGCTACTGCTCGTAA\
    ATTTGCCAGAGAGGAAATCATCCCAGTGGCTGCAGAATATGATAAAACTGGTGAATATCCAGTCCCCCTAATTAGAAGAG\
    CCTGGGAACTTGGTTTAATGAACACACACATTCCAGAGAACTGTGGAGGTCTTGGACTTGGAACTTTTGATGCTTGTTTA\
    ATTAGTGAAGAATTGGCTTATGGATGTACAGGGGTTCAGACTGCTATTGAAGGAAATTCTTTGGGGCAAATGCCTATTAT\
    TATTGCTGGAAATGATCAACAAAAGAAGAAGTATTTGGGGAGAATGACTGAGGAGCCATTGATGTGTGCTTATTGTGTAA\
    CAGAACCTGGAGCAGGCTCTGATGTAGCTGGTATAAAGACCAAAGCAGAAAAGAAAGGAGATGAGTATATTATTAATGGT\
    CAGAAGATGTGGATAACCAACGGAGGAAAAGCTAATTGGTATTTTTTATTGGCACGTTCTGATCCAGATCCTAAAGCTCC\
    TGCTAATAAAGCCTTTACTGGATTCATTGTGGAAGCAGATACCCCAGGAATTCAGATTGGGAGAAAGGAATTAAACATGG\
    GCCAGCGATGTTCAGATACTAGAGGAATTGTCTTCGAAGATGTGAAAGTGCCTAAAGAAAATGTTTTAATTGGTGACGGA\
    GCTGGTTTCAAAGTTGCAATGGGAGCTTTTGATAAAACCAGACCTGTAGTAGCTGCTGGTGCTGTTGGATTAGCACAAAG\
    AGCTTTGGATGAAGCTACCAAGTATGCCCTGGAAAGGAAAACTTTCGGAAAGCTACTTGTAGAGCACCAAGCAATATCAT\
    TTATGCTGGCTGAAATGGCAATGAAAGTTGAACTAGCTAGAATGAGTTACCAGAGAGCAGCTTGGGAGGTTGATTCTGGT\
    CGTCGAAATACCTATTATGCTTCTATTGCAAAGGCATTTGCTGGAGATATTGCAAATCAGTTAGCTACTGATGCTGTGCA\
    GATACTTGGAGGCAATGGATTTAATACAGAATATCCTGTAGAAAAACTAATGAGGGATGCCAAAATCTATCAGATTTATG\
    AAGGTACTTCACAAATTCAAAGACTTATTGTAGCCCGTGAACACATTGACAAGTACAAAAATTAAAAAAATTACTGTAGA\
    AATATTGAATAACTAGAACACAAGCCACTGTTTCAGCTCCAGAAAAAAGAAAGGGCTTTAACGTTTTTTCCAGTGAAAAC\
    AAATCCTCTTATATTAAATCTAAGCAACTGCTTATTATAGTAGTTTATACTTTTGCTTAACTCTGTTATGTCTCTTAAGC\
    AGGTTTGGTTTTTATTAAAATGATGTGTTTTCTTTAGTACCACTTTACTTGAATTACATTAACCTAGAAAACTACATAGG\
    TTATTTTGATCTCTTAAGATTAATGTAGCAGAAATTTCTTGGAATTTTATTTTTGTAATGACAGAAAAGTGGGCTTAGAA\
    AGTATTCAAGATGTTACAAAATTTACATTTAGAAAATATTGTAGTATTTGAATACTGTCAACTTGACAGTAACTTTGTAG\
    ACTTAATGGTATTATTAAAGTTCTTTTTATTGCAGTTTGGAAAGCATTTGTGAAACTTTCTGTTTGGCACAGAAACAGTC\
    AAAATTTTGACATTCATATTCTCCTATTTTACAGCTACAAGAACTTTCTTGAAAATCTTATTTAATTCTGAGCCCATATT\
    TCACTTACCTTATTTAAAATAAATCAATAAAGCTTGCCTTAAATTATTTTTATATGACTGTTGGTCTCTAGGTAGCCTTT\
    GGTCTATTGTACACAATCTCATTTCATATGTTTGCATTTTGGCAAAGAACTTAATAAAATTGTTCAGTGCTTATTATCAT\
    ATCTTTCTGTATTTTTTCCAGGAAATTTCATTACTTCGTGTAATAGTGTATATTTCTTGTATTTACTATGATGAAAAAAG\
    GTCGTTTTAATTTTGAATTGAATAAAGTTACCTGTTCATTTTTTATTAGATATTTTAAAGACTTCAGAAAATATAAATAT\
    GAAATAATTTAAGAACCCAAA\
    >NM_000017.4 Homo sapiens acyl-CoA dehydrogenase short chain (ACADS), transcript variant 1, mRNA; nuclear gene for mitochondrial product\
    ACTCCGGAACAGCGCGCTCGCAGCGGGAGGTCGCGAAGCCTGGGACTGTGTCTGTCGCCCATGGCCGCCGCGCTGCTCGC\
    CCGGGCCTCGGGCCCTGCCCGCAGAGCTCTCTGTCCTAGGGCCTGGCGGCAGTTACACACCATCTACCAGTCTGTGGAAC\
    TGCCCGAGACACACCAGATGTTGCTCCAGACATGCCGGGACTTTGCCGAGAAGGAGTTGTTTCCCATTGCAGCCCAGGTG\
    GATAAGGAACATCTCTTCCCAGCGGCTCAGGTGAAGAAGATGGGCGGGCTTGGGCTTCTGGCCATGGACGTGCCCGAGGA\
    GCTTGGCGGTGCTGGCCTCGATTACCTGGCCTACGCCATCGCCATGGAGGAGATCAGCCGTGGCTGCGCCTCCACCGGAG\
    TCATCATGAGTGTCAACAACTCTCTCTACCTGGGGCCCATCTTGAAGTTTGGCTCCAAGGAGCAGAAGCAGGCGTGGGTC\
    ACGCCTTTCACCAGTGGTGACAAAATTGGCTGCTTTGCCCTCAGCGAACCAGGGAACGGCAGTGATGCAGGAGCTGCGTC\
    CACCACCGCCCGGGCCGAGGGCGACTCATGGGTTCTGAATGGAACCAAAGCCTGGATCACCAATGCCTGGGAGGCTTCGG\
    CTGCCGTGGTCTTTGCCAGCACGGACAGAGCCCTGCAAAACAAGGGCATCAGTGCCTTCCTGGTCCCCATGCCAACGCCT\
    GGGCTCACGTTGGGGAAGAAAGAAGACAAGCTGGGCATCCGGGGCTCATCCACGGCCAACCTCATCTTTGAGGACTGTCG\
    CATCCCCAAGGACAGCATCCTGGGGGAGCCAGGGATGGGCTTCAAGATAGCCATGCAAACCCTGGACATGGGCCGCATCG\
    GCATCGCCTCCCAGGCCCTGGGCATTGCCCAGACCGCCCTCGATTGTGCTGTGAACTACGCTGAGAATCGCATGGCCTTC\
    GGGGCGCCCCTCACCAAGCTCCAGGTCATCCAGTTCAAGTTGGCAGACATGGCCCTGGCCCTGGAGAGTGCCCGGCTGCT\
    GACCTGGCGCGCTGCCATGCTGAAGGATAACAAGAAGCCTTTCATCAAGGAGGCAGCCATGGCCAAGCTGGCCGCCTCGG\
    AGGCCGCGACCGCCATCAGCCACCAGGCCATCCAGATCCTGGGCGGCATGGGCTACGTGACAGAGATGCCGGCAGAGCGG\
    CACTACCGCGACGCCCGCATCACTGAGATCTACGAGGGCACCAGCGAAATCCAGCGGCTGGTGATCGCCGGGCATCTGCT\
    CAGGAGCTACCGGAGCTGAGCCCGCGGCGGACTGCCCCAGGACTGCGGGAAGGCGCGGGAGCCAGGGGCCTCCACCCCAA\
    CCCCGGCTCAGAGACTGGGCGGCCCGGCGGGGGCTCCCTGGGGACCCCAGATGGGCTCAGTGCTGCCACCCAGATCAGAT\
    CACATGGGAATGAGGCCCTCCGACCATTGGCAGCTCCGCCTCTGGGCCTTTCCGCCTCCTCACCACTGTGCCTCAAGTTC\
    CTCATCTAAGTGGCCCTGGCCTC"; // 65 lines (last line incomplete = 64)

// Third block of the genome file
const THIRD_GENOME_BLOCK: &str = 
    "CTGGGGGCGGGGTTGTGGGGGGGCTGAGCGACACTCAGGGACACCTCAGTTGTCCTC\
    CCGCGGGCCCTGGTGCCCTGGCATGAAGGCCCAGTGCGACAGGCCCTTGGTGGGGTCTGTCTTTTCCTTGAGGTCAGAGG\
    TCAGGAGCAGGGCTGGGGTCAGGATGACGAGGCCTGGGGTCCTGGTGTTGGGCAGGTGGTGGGGCTGGGCCATGGAGCTG\
    GCCCAGAGGCCCCTCAGCCCTTTGTAAAGTCTGATGAAGGCAGGGGTGGTGATTCATGCTGTGTGACTGACTGTGGGTAA\
    TAAACACACCTGTCCCCCA\
    >NM_000018.4 Homo sapiens acyl-CoA dehydrogenase very long chain (ACADVL), transcript variant 1, mRNA; nuclear gene for mitochondrial product\
    AGAGCTGGGTCAGAGCTCGAGCCAGCGGCGCCCGGAGAGATTCGGAGATGCAGGCGGCTCGGATGGCCGCGAGCTTGGGG\
    CGGCAGCTGCTGAGGCTCGGGGGCGGAAGCTCGCGGCTCACGGCGCTCCTGGGGCAGCCCCGGCCCGGCCCTGCCCGGCG\
    GCCCTATGCCGGGGGTGCCGCTCAGCTGGCTCTGGACAAGTCAGATTCCCACCCCTCTGACGCTCTGACCAGGAAAAAAC\
    CGGCCAAGGCGGAATCTAAGTCCTTTGCTGTGGGAATGTTCAAAGGCCAGCTCACCACAGATCAGGTGTTCCCATACCCG\
    TCCGTGCTCAACGAAGAGCAGACACAGTTTCTTAAAGAGCTGGTGGAGCCTGTGTCCCGTTTCTTCGAGGAAGTGAACGA\
    TCCCGCCAAGAATGACGCTCTGGAGATGGTGGAGGAGACCACTTGGCAGGGCCTCAAGGAGCTGGGGGCCTTTGGTCTGC\
    AAGTGCCCAGTGAGCTGGGTGGTGTGGGCCTTTGCAACACCCAGTACGCCCGTTTGGTGGAGATCGTGGGCATGCATGAC\
    CTTGGCGTGGGCATTACCCTGGGGGCCCATCAGAGCATCGGTTTCAAAGGCATCCTGCTCTTTGGCACAAAGGCCCAGAA\
    AGAAAAATACCTCCCCAAGCTGGCATCTGGGGAGACTGTGGCCGCTTTCTGTCTAACCGAGCCCTCAAGCGGGTCAGATG\
    CAGCCTCCATCCGAACCTCTGCTGTGCCCAGCCCCTGTGGAAAATACTATACCCTCAATGGAAGCAAGCTTTGGATCAGT\
    AATGGGGGCCTAGCAGACATCTTCACGGTCTTTGCCAAGACACCAGTTACAGATCCAGCCACAGGAGCCGTGAAGGAGAA\
    GATCACAGCTTTTGTGGTGGAGAGGGGCTTCGGGGGCATTACCCATGGGCCCCCTGAGAAGAAGATGGGCATCAAGGCTT\
    CAAACACAGCAGAGGTGTTCTTTGATGGAGTACGGGTGCCATCGGAGAACGTGCTGGGTGAGGTTGGGAGTGGCTTCAAG\
    GTTGCCATGCACATCCTCAACAATGGAAGGTTTGGCATGGCTGCGGCCCTGGCAGGTACCATGAGAGGCATCATTGCTAA\
    GGCGGTAGATCATGCCACTAATCGTACCCAGTTTGGGGAGAAAATTCACAACTTTGGGCTGATCCAGGAGAAGCTGGCAC\
    GGATGGTTATGCTGCAGTATGTAACTGAGTCCATGGCTTACATGGTGAGTGCTAACATGGACCAGGGAGCCACGGACTTC\
    CAGATAGAGGCCGCCATCAGCAAAATCTTTGGCTCGGAGGCAGCCTGGAAGGTGACAGATGAATGCATCCAAATCATGGG\
    GGGTATGGGCTTCATGAAGGAACCTGGAGTAGAGCGTGTGCTCCGAGATCTTCGCATCTTCCGGATCTTTGAGGGGACAA\
    ATGACATTCTTCGGCTGTTTGTGGCTCTGCAGGGCTGTATGGACAAAGGAAAGGAGCTCTCTGGGCTTGGCAGTGCTCTA\
    AAGAATCCCTTTGGGAATGCTGGCCTCCTGCTAGGAGAGGCAGGCAAACAGCTGAGGCGGCGGGCAGGGCTGGGCAGCGG\
    CCTGAGTCTCAGCGGACTTGTCCACCCGGAGTTGAGTCGGAGTGGCGAGCTGGCAGTACGGGCTCTGGAGCAGTTTGCCA\
    CTGTGGTGGAGGCCAAGCTGATAAAACACAAGAAGGGGATTGTCAATGAACAGTTTCTGCTGCAGCGGCTGGCAGACGGG\
    GCCATCGACCTCTATGCCATGGTGGTGGTTCTCTCGAGGGCCTCAAGATCCCTGAGTGAGGGCCACCCCACGGCCCAGCA\
    TGAGAAAATGCTCTGTGACACCTGGTGTATCGAGGCTGCAGCTCGGATCCGAGAGGGCATGGCCGCCCTGCAGTCTGACC\
    CCTGGCAGCAAGAGCTCTACCGCAACTTCAAAAGCATCTCCAAGGCCTTGGTGGAGCGGGGTGGTGTGGTCACCAGCAAC\
    CCACTTGGCTTCTGAATACTCCCGGCCAGGGCCTGTCCCAGTTATGTGCCTTCCCTCAAGCCAAAGCCGAAGCCCCTTTC\
    CTTAAGGCCCTGGTTTGTCCCGAAGGGGCCTAGTGTTCCCAGCACTGTGCCTGCTCTCAAGAGCACTTACTGCCTCGCAA\
    ATAATAAAAATTTCTAGCCAGTCA\
    >NM_000019.4 Homo sapiens acetyl-CoA acetyltransferase 1 (ACAT1), transcript variant 2, mRNA; nuclear gene for mitochondrial product\
    AGTCTACGCCTGTGGAGCCGATACTCAGCCCTCTGCGACCATGGCTGTGCTGGCGGCACTTCTGCGCAGCGGCGCCCGCA\
    GCCGCAGCCCCCTGCTCCGGAGGCTGGTGCAGGAAATAAGATATGTGGAACGGAGTTATGTATCAAAACCCACTTTGAAG\
    GAAGTGGTCATAGTAAGTGCTACAAGAACACCCATTGGATCTTTTTTAGGCAGCCTTTCCTTGCTGCCAGCCACTAAGCT\
    TGGTTCCATTGCAATTCAGGGAGCCATTGAAAAGGCAGGGATTCCAAAAGAAGAAGTGAAAGAAGCATACATGGGTAATG\
    TTCTACAAGGAGGTGAAGGACAAGCTCCTACAAGGCAGGCAGTATTGGGTGCAGGCTTACCTATTTCTACTCCATGTACC\
    ACCATAAACAAAGTTTGTGCTTCAGGAATGAAAGCCATCATGATGGCCTCTCAAAGTCTTATGTGTGGACATCAGGATGT\
    GATGGTGGCAGGTGGGATGGAGAGCATGTCCAATGTTCCATATGTAATGAACAGAGGATCAACACCATATGGTGGGGTAA\
    AGCTTGAAGATTTGATTGTAAAAGACGGGCTAACTGATGTCTACAATAAAATTCATATGGGCAGCTGTGCTGAGAATACA\
    GCAAAGAAGCTGAATATTGCACGAAATGAACAGGACGCTTATGCTATTAATTCTTATACCAGAAGTAAAGCAGCATGGGA\
    AGCTGGGAAATTTGGAAATGAAGTTATTCCTGTCACAGTTACAGTAAAAGGTCAACCAGATGTAGTGGTGAAAGAAGATG\
    AAGAATATAAACGTGTTGATTTTAGCAAAGTTCCAAAGCTGAAGACAGTTTTCCAGAAAGAAAATGGCACAGTAACAGCT\
    GCCAATGCCAGTACACTGAATGATGGAGCAGCTGCTCTGGTTCTCATGACGGCAGATGCAGCGAAGAGGCTCAATGTTAC\
    ACCACTGGCAAGAATAGTAGCATTTGCTGACGCTGCTGTAGAACCTATTGATTTTCCAATTGCTCCTGTATATGCTGCAT\
    CTATGGTTCTTAAAGATGTGGGATTGAAAAAAGAAGATATTGCAATGTGGGAAGTAAATGAAGCCTTTAGTCTGGTTGTA\
    CTAGCAAACATTAAAATGTTGGAGATTGATCCCCAAAAAGTGAATATCAATGGAGGAGCTGTTTCTCTGGGACATCCAAT\
    TGGGATGTCTGGAGCCAGGATTGTTGGTCATTTGACTCATGCCTTGAAGCAAGGAGAATACGGTCTTGCCAGTATTTGCA\
    ATGGAGGAGGAGGTGCTTCTGCCATGCTAATTCAGAAGCTGTAGACAACCTCTGCTATTTAAGGAGACAACCCTATGTGA\
    CCAGAAGGCCTGCTGTAATCAGTGTGACTACTGTGGGTCAGCTTATATTCAGATAAGCTGTTTCATTTTTTATTATTTTC\
    TATGTTAACTTTTAAAAATCAAAATGATGAAATCCCAAAACATTTTGAAATTAAAAATAAATTTCTTCTTCTGCTTTTTT\
    CTTGGTAACCTTGAAAA\
    >NM_000020.3 Homo sapiens activin A receptor like type 1 (ACVRL1), transcript variant 1, mRNA\
    CCCAGTCCCGGGAGGCTGCCGCGCCAGCTGCGCCGAGCGAGCCCCTCCCCGGCTCCAGCCCGGTCCGGGGCCGCGCCCGG\
    ACCCCAGCCCGCCGTCCAGCGCTGGCGGTGCAACTGCGGCCGCGCGGTGGAGGGGAGGTGGCCCCGGTCCGCCGAAGGCT\
    AGCGCCCCGCCACCCGCAGAGCGGGCCCAGAGGGACCATGACCTTGGGCTCCCCCAGGAAAGGCCTTCTGATGCTGCTGA\
    TGGCCTTGGTGACCCAGGGAGACCCTGTGAAGCCGTCTCGGGGCCCGCTGGTGACCTGCACGTGTGAGAGCCCACATTGC\
    AAGGGGCCTACCTGCCGGGGGGCCTGGTGCACAGTAGTGCTGGTGCGGGAGGAGGGGAGGCACCCCCAGGAACATCGGGG\
    CTGCGGGAACTTGCACAGGGAGCTCTGCAGGGGGCGCCCCACCGAGTTCGTCAACCACTACTGCTGCGACAGCCACCTCT\
    GCAACCACAACGTGTCCCTGGTGCTGGAGGCCACCCAACCTCCTTCGGAGCAGCCGGGAACAGATGGCCAGCTGGCCCTG\
    ATCCTGGGCCCCGTGCTGGCCTTGCTGGCCCTGGTGGCCCTGGGTGTCCTGGGCCTGTGGCATGTCCGACGGAGGCAGGA\
    GAAGCAGCGTGGC"; // 65 lines (last line incomplete = 64)

const FOURTH_BLOCK_GENOME: &str =
    "CTGCACAGCGAGCTGGGAGAGTCCAGTCTCATCCTGAAAGCATCTGAGCAGGGCGACAGCATGTTGG\
    GGGACCTCCTGGACAGTGACTGCACCACAGGGAGTGGCTCAGGGCTCCCCTTCCTGGTGCAGAGGACAGTGGCACGGCAG\
    GTTGCCTTGGTGGAGTGTGTGGGAAAAGGCCGCTATGGCGAAGTGTGGCGGGGCTTGTGGCACGGTGAGAGTGTGGCCGT\
    CAAGATCTTCTCCTCGAGGGATGAACAGTCCTGGTTCCGGGAGACTGAGATCTATAACACAGTGTTGCTCAGACACGACA\
    ACATCCTAGGCTTCATCGCCTCAGACATGACCTCCCGCAACTCGAGCACGCAGCTGTGGCTCATCACGCACTACCACGAG\
    CACGGCTCCCTCTACGACTTTCTGCAGAGACAGACGCTGGAGCCCCATCTGGCTCTGAGGCTAGCTGTGTCCGCGGCATG\
    CGGCCTGGCGCACCTGCACGTGGAGATCTTCGGTACACAGGGCAAACCAGCCATTGCCCACCGCGACTTCAAGAGCCGCA\
    ATGTGCTGGTCAAGAGCAACCTGCAGTGTTGCATCGCCGACCTGGGCCTGGCTGTGATGCACTCACAGGGCAGCGATTAC\
    CTGGACATCGGCAACAACCCGAGAGTGGGCACCAAGCGGTACATGGCACCCGAGGTGCTGGACGAGCAGATCCGCACGGA\
    CTGCTTTGAGTCCTACAAGTGGACTGACATCTGGGCCTTTGGCCTGGTGCTGTGGGAGATTGCCCGCCGGACCATCGTGA\
    ATGGCATCGTGGAGGACTATAGACCACCCTTCTATGATGTGGTGCCCAATGACCCCAGCTTTGAGGACATGAAGAAGGTG\
    GTGTGTGTGGATCAGCAGACCCCCACCATCCCTAACCGGCTGGCTGCAGACCCGGTCCTCTCAGGCCTAGCTCAGATGAT\
    GCGGGAGTGCTGGTACCCAAACCCCTCTGCCCGACTCACCGCGCTGCGGATCAAGAAGACACTACAAAAAATTAGCAACA\
    GTCCAGAGAAGCCTAAAGTGATTCAATAGCCCAGGAGCACCTGATTCCTTTCTGCCTGCAGGGGGCTGGGGGGGTGGGGG\
    GCAGTGGATGGTGCCCTATCTGGGTAGAGGTAGTGTGAGTGTGGTGTGTGCTGGGGATGGGCAGCTGCGCCTGCCTGCTC\
    GGCCCCCAGCCCACCCAGCCAAAAATACAGCTGGGCTGAAACCTGATCCCCTGCTGTCTGGCCTGCTCAAAGCGGCAGGC\
    TCCCTGACGCCTGGCTCTCTCCCCACCCCTATGGCCAGCATGGTGCACCCCCTACCACTCCCGGGACAGGATGCAAAAGA\
    GGCTCCAGAGTCAGAGTGCCAAGCCAGGGAATCCCAGTCCCAGACTCAGAGCCCGGGCCTGCACTTTGCCCCCTGCCCTT\
    GATCAACCCCACTGCCCCACCAGAGCTGCCAGGGTGGCACAGGGCCCTGTCCAGCCCCTGGCACACACTTCCCTGCCAGG\
    CCTCAGCCTCTAGCATAAGCTCCAGAGAGCCAGGGCCCATCAGTTTCTCTCTGTGGATTTGTATCTCAGCTCCATGATGC\
    CTTGGGCTTTCTGTCTCCTCAACAAGAGTGCAGCTTGCTGAATGTCAGCTGCCTGAGAGAGCTGGGGCCTGACTTACTAG\
    GGCATTAAATCCTAAGAGGTCCTACTGAGGTGTGGCAGGATCACAGGCCAGTGGAAAAAGGGCAGGTCAGATGGGCAAGG\
    CCCAGGACTTTCAGATTAACTGAGAGGATATCGAGGCCAAGCATGGCAGGGGGAAGGTCAGTGGGTGTCAAGAGACCCAG\
    GTCTGACCCCGGATGTTTGCTCCATGTGACAAAAGCAGGCCTGTCTCAGGACCTTTTCTTTTCTTTTTTCCTTCTTTTTT\
    TTTTTGACACGGAGTTTCGCTCTTGTTGTCCAGGCTAGAGTGCAATGGCATGATCCCAGCTCACCGCAACGTCTACCTCC\
    CAGGTTCAAATCATTCTCTTGCCTCAGACTCCCGAGTAGCTGGGATTACAGGCACATGCCACCATGCCTGGCTAATTTTG\
    TATATTTAGTAGAAACAGGGTTTCACCATGCTGGCCATGCTGGTCTCGAACTCCTGACCTCAGGTGTTCCACCTACCTCA\
    GCCTCCCAAAGTGCTGGGGTTACAGGTGTGAGCCATCGCGCCTGGCCAGGACCTTTGTTTCTTATCTACATATTGGAAGA\
    TTTGGTCCTGATGTCCTTTGAGGCTTCTTTAGCTCTAGTTCTCTGACACTTCAGCCTATATCACAGCTAACTTCTTCAGT\
    CTCATCTATTCCTTATGCTCCAGCCCCTGGCAATTTGCCTCAAGATGGGGGTTTGAAAATAACTTTACCTGACTCAAGGA\
    GTGTCTGGAGCACCTCCTAGTCTAAGTCTGCAAGCTCCAGTTCTTGCCTAAAACCATGCCAGTGGCCACCCTTGGGCTCA\
    GACAGCTCTGGGCCTTTTGACCACAAGCCAGCCCCTCGCCCTCTCTGTGGCATAGTCTTCTCTGCCCCAGGACTGCAGGG\
    CGGCTTCCTCCAAGGCTTCCAAGGCTCAAAAGAAATTTGGCTCCATCCAAGAAGGCTCCAGCTCCCCTACTGGCCCCTGG\
    CTCAGGCCCACACCCCTGGCCAGGCCCAGAGAGTGTGTCTCAGGAGAATTCAATGGCTCTAGAGAGACACACAGAAAGTT\
    TGGCATTTGGAAATTTCAAGGATGTATGTATGCTCACGTATGGAGCAGGTTGTCCTGGTCCCTGGGTGCAGGGAAGTGGG\
    CTGCAGGGAAGTGGATTGGAGGGGAGCTTGAGGAATATAAGGAGCGGGGGTGGAGACTCAGGCTATGGACAAGGACAGCC\
    CCAAGGTTGGGAAGACCTGGCCTTAGTCGTCCTCAGCCTAGGGGCAGGGCAGTGAAGAAAGCTCTCCCCGCTCCTGCTGT\
    AATGACCCAGAGTAGCCTCCCCAGGCCGGCATCTTATGTGTGTCTTCCACCATCCTCATGGTGGCACTTTTCTAGGCCTG\
    TCTCCCAGCATTGTGCAAGGCTCGGAAGAGAACCAGGAAGTGAAACTGGGTGAAAACAGAAAGCTCAATGGATGGGCTAG\
    GTTCCCAGATCATTAGGGCAGAGTTTGCACGTCCTCTGGTCACTGGAATCCACCCAGCCCACGAATCATCTCCCTCTTGA\
    AGGATTTTATTTCTACTGGGTTTTGGAACAAACTCCTGCTGAGACCCCACAGCCAGAAACTGAAAGCAGCAGCTCCCCAA\
    AGCCTGGAAAATCCCTAAGAGAAGGCCTGGGGCAGGAAGTGGAGTGACAGGGGACAGGTAGAGAGAAGGGGGCCCAATGG\
    CCAGGGAGTGAAGGAGGTGGCGTTGCTGAGAGCAGTCTGCACATGCTTCTGTCTGAGTGCAGGAAGGTGTTCCAGGGTCG\
    AAATTACACTTCTCGTACCTGGAGACGCTGTTTGTGGGAGCACTGGGCTCATGCCTGGCACACAATAGGTCTGCAATAAA\
    CCATGGTTAAATCCTGA\
    >NM_000021.4 Homo sapiens presenilin 1 (PSEN1), transcript variant 1, mRNA\
    GGAAACAAAACAGCGGCTGGTCTGGAAGGAACCTGAGCTACGAGCCGCGGCGGCAGCGGGGCGGCGGGGAAGCGTATACC\
    TAATCTGGGAGCCTGCAAGTGACAACAGCCTTTGCGGTCCTTAGACAGCTTGGCCTGGAGGAGAACACATGAAAGAAAGA\
    ACCTCAAGAGGCTTTGTTTTCTGTGAAACAGTATTTCTATACAGTTGCTCCAATGACAGAGTTACCTGCACCGTTGTCCT\
    ACTTCCAGAATGCACAGATGTCTGAGGACAACCACCTGAGCAATACTGTACGTAGCCAGAATGACAATAGAGAACGGCAG\
    GAGCACAACGACAGACGGAGCCTTGGCCACCCTGAGCCATTATCTAATGGACGACCCCAGGGTAACTCCCGGCAGGTGGT\
    GGAGCAAGATGAGGAAGAAGATGAGGAGCTGACATTGAAATATGGCGCCAAGCATGTGATCATGCTCTTTGTCCCTGTGA\
    CTCTCTGCATGGTGGTGGTCGTGGCTACCATTAAGTCAGTCAGCTTTTATACCCGGAAGGATGGGCAGCTAATCTATACC\
    CCATTCACAGAAGATACCGAGACTGTGGGCCAGAGAGCCCTGCACTCAATTCTGAATGCTGCCATCATGATCAGTGTCAT\
    TGTTGTCATGACTATCCTCCTGGTGGTTCTGTATAAATACAGGTGCTATAAGGTCATCCATGCCTGGCTTATTATATCAT\
    CTCTATTGTTGCTGTTCTTTTTTTCATTCATTTACTTGGGGGAAGTGTTTAAAACCTATAACGTTGCTGTGGACTACATT\
    ACTGTTGCACTCCTGATCTGGAATTTTGGTGTGGTGGGAATGATTTCCATTCACTGGAAAGGTCCACTTCGACTCCAGCA\
    GGCATATCTCATTATGATTAGTGCCCTCATGGCCCTGGTGTTTATCAAGTACCTCCCTGAATGGACTGCGTGGCTCATCT\
    TGGCTGTGATTTCAGTATATGATTTAGTGGCTGTTTTGTGTCCGAAAGGTCCACTTCGTATGCTGGTTGAAACAGCTCAG\
    GAGAGAAATGAAACGCTTTTTCCAGCTCTCATTTACTCCTCAACAATGGTGTGGTTGGTGAATATGGCAGAAGGAGACCC\
    GGAAGCTCAAAGGAGAGTATCCAAAAATTCCAAGTATAATGCAGAAAGCACAGAAAGGGAGTCACAAGACACTGTTGCAG\
    AGAATGATGATGGCGGGTTCAGTGAGGAATGGGAAGCCCAGAGGGACAGTCATCTAGGGCCTCATCGCTCTACACCTGAG\
    TCACGAGCTGCTGTCCAGGAACTTTCCAGCAGTATCCTCGCTGGTGAAGACCCAGAGGAAAGGGGAGTAAAACTTGGATT\
    GGGAGATTTCATTTTCTACAGTGTTCTGGTTGGTAAAGCCTCAGCAACAGCCAGTGGAGACTGGAACACAACCATAGCCT\
    GTTTCGTAGCCATATTAA"; // 65 lines (last line incomplete = 64)

/*
===========================================
			SELECTION CONSTANTS
===========================================
*/

// The result of deleting a ~2400 line selection from the end of the GENOME_FILE
const DELETED_BLOCKS_RESULT: &str =
	">NM_000014.6 Homo sapiens alpha-2-macroglobulin (A2M), transcript variant 1, mRNA\
	GGGACCAGATGGATTGTAGGGAGTAGGGTACAATACAGTCTGTTCTCCTCCAGCTCCTTCTTTCTGCAACATGGGGAAGA\
	ACAAACTCCTTCATCCAAGTCTGGTTCTTCTCCTCTTGGTCCTCCTGCCCACAGACGCCTCAGTCTCTGGAAAACCGCAG\
	TATATGGTTCTGGTCCCCTCCCTGCTCCACACTGAGACCACTGAGAAGGGCTGTGTCCTTCTGAGCTACCTGAATGAGAC\
	AGTGACTGTAAGTGCTTCCTTGGAGTCTGTCAGGGGAAACAGGAGCCTCTTCACTGACCTGGAGGCGGAGAATGACGTAC\
	TCCACTGTGTCGCCTTCGCTGTCCCAAAGTCTTCATCCAATGAGGAGGTAATGTTCCTCACTGTCCAAGTGAAAGGACCA\
	ACCCAAGAATTTAAGAAGCGGACCACAGTGATGGTTAAGAACGAGGACAGTCTGGTCTTTGTCCAGACAGACAAATCAAT\
	CTACAAACCAGGGCAGACAGTGAAATTTCGTGTTGTCTCCATGGATGAAAACTTTCACCCCCTGAATGAGTTGATTCCAC\
	TAGTATACATTCAGGATCCCAAAGGAAATCGCATCGCACAATGGCAGAGTTTCCAGTTAGAGGGTGGCCTCAAGCAATTT\
	TCTTTTCCCCTCTCATCAGAGCCCTTCCAGGGCTCCTACAAGGTGGTGGTACAGAAGAAATCAGGTGGAAGGACAGAGCA\
	CCCTTTCACCGTGGAGGAATTTGTTCTTCCCAAGTTTGAAGTACAAGTAACAGTGCCAAAGATAATCACCATCTTGGAAG\
	AAGAGATGAATGTATCAGTGTGTGGCCTATACACATATGGGAAGCCTGTCCCTGGACATGTGACTGTGAGCATTTGCAGA\
	AAGTATAGTGACGCTTCCGACTGCCACGGTGAAGATTCACAGGCTTTCTGTGAGAAATTCAGTGGACAGCTAAACAGCCA\
	TGGCTGCTTCTATCAGCAAGTAAAAACCAAGGTCTTCCAGCTGAAGAGGAAGGAGTATGAAATGAAACTTCACACTGAGG\
	CCCAGATCCAAGAAGAAGGAACAGTGGTGGAATTGACTGGAAGGCAGTCCAGTGAAATCACAAGAACCATAACCAAACTC\
	TCATTTGTGAAAGTGGACTCACACTTTCGACAGGGAATTCCCTTCTTTGGGCAGGTGCGCCTAGTAGATGGGAAAGGCGT\
	CCCTATACCAAATAAAGTCATATTCATCAGAGGAAATGAAGCAAACTATTACTCCAATGCTACCACGGATGAGCATGGCC\
	TTGTACAGTTCTCTATCAACACCACCAATGTTATGGGTACCTCTCTTACTGTTAGGGTCAATTACAAGGATCGTAGTCCC\
	TGTTACGGCTACCAGTGGGTGTCAGAAGAACACGAAGAGGCACATCACACTGCTTATCTTGTGTTCTCCCCAAGCAAGAG\
	CTTTGTCCACCTTGAGCCCATGTCTCATGAACTACCCTGTGGCCATACTCAGACAGTCCAGGCACATTATATTCTGAATG\
	GAGGCACCCTGCTGGGGCTGAAGAAGCTCTCCTTCTATTATCTGATAATGGCAAAGGGAGGCATTGTCCGAACTGGGACT\
	CATGGACTGCTTGTGAAGCAGGAAGACATGAAGGGCCATTTTTCCATCTCAATCCCTGTGAAGTCAGACATTGCTCCTGT\
	CGCTCGGTTGCTCATCTATGCTGTTTTACCTACCGGGGACGTGATTGGGGATTCTGCAAAATATGATGTTGAAAATTGTC\
	TGGCCAACAAGGTGGATTTGAGCTTCAGCCCATCACAAAGTCTCCCAGCCTCACACGCCCACCTGCGAGTCACAGCGGCT\
	CCTCAGTCCGTCTGCGCCCTCCGTGCTGTGGACCAAAGCGTGCTGCTCATGAAGCCTGATGCTGAGCTCTCGGCGTCCTC\
	GGTTTACAACCTGCTACCAGAAAAGGACCTCACTGGCTTCCCTGGGCCTTTGAATGACCAGGACAATGAAGACTGCATCA\
	ATCGTCATAATGTCTATATTAATGGAATCACATATACTCCAGTATCAAGTACAAATGAAAAGGATATGTACAGCTTCCTA\
	GAGGACATGGGCTTAAAGGCATTCACCAACTCAAAGATTCGTAAACCCAAAATGTGTCCACAGCTTCAACAGTATGAAAT\
	GCATGGACCTGAAGGTCTACGTGTAGGTTTTTATGAGTCAGATGTAATGGGAAGAGGCCATGCACGCCTGGTGCATGTTG\
	AAGAGCCTCACACGGAGACCGTACGAAAGTACTTCCCTGAGACATGGATCTGGGATTTGGTGGTGGTAAACTCAGCAGGT\
	GTGGCTGAGGTAGGAGTAACAGTCCCTGACACCATCACCGAGTGGAAGGCAGGGGCCTTCTGCCTGTCTGAAGATGCTGG\
	ACTTGGTATCTCTTCCACTGCCTCTCTCCGAGCCTTCCAGCCCTTCTTTGTGGAGCTCACAATGCCTTACTCTGTGATTC\
	GTGGAGAGGCCTTCACACTCAAGGCCACGGTCCTAAACTACCTTCCCAAATGCATCCGGGTCAGTGTGCAGCTGGAAGCC\
	TCTCCCGCCTTCCTAGCTGTCCCAGTGGAGAAGGAACAAGCGCCTCACTGCATCTGTGCAAACGGGCGGCAAACTGTGTC\
	CTGGGCAGTAACCCCAAAGTCATTAGGAAATGTGAATTTCACTGTGAGCGCAGAGGCACTAGAGTCTCAAGAGCTGTGTG\
	GGACTGAGGTGCCTTCAGTTCCTGAACACGGAAGGAAAGACACAGTCATCAAGCCTCTGTTGGTTGAACCTGAAGGACTA\
	GAGAAGGAAACAACATTCAACTCCCTACTTTGTCCATCAGGTGGTGAGGTTTCTGAAGAATTATCCCTGAAACTGCCACC\
	AAATGTGGTAGAAGAATCTGCCCGAGCTTCTGTCTCAGTTTTGGGAGACATATTAGGCTCTGCCATGCAAAACACACAAA\
	ATCTTCTCCAGATGCCCTATGGCTGTGGAGAGCAGAATATGGTCCTCTTTGCTCCTAACATCTATGTACTGGATTATCTA\
	AATGAAACACAGCAGCTTACTCCAGAGATCAAGTCCAAGGCCATTGGCTATCTCAACACTGGTTACCAGAGACAGTTGAA\
	CTACAAACACTATGATGGCTCCTACAGCACCTTTGGGGAGCGATATGGCAGGAACCAGGGCAACACCTGGCTCACAGCCT\
	TTGTTCTGAAGACTTTTGCCCAAGCTCGAGCCTACATCTTCATCGATGAAGCACACATTACCCAAGCCCTCATATGGCTC\
	TCCCAGAGGCAGAAGGACAATGGCTGTTTCAGGAGCTCTGGGTCACTGCTCAACAATGCCATAAAGGGAGGAGTAGAAGA\
	TGAAGTGACCCTCTCCGCCTATATCACCATCGCCCTTCTGGAGATTCCTCTCACAGTCACTCACCCTGTTGTCCGCAATG\
	CCCTGTTTTGCCTGGAGTCAGCCTGGAAGACAGCACAAGAAGGGGACCATGGCAGCCATGTATATACCAAAGCACTGCTG\
	GCCTATGCTTTTGCCCTGGCAGGTAACCAGGACAAGAGGAAGGAAGTACTCAAGTCACTTAATGAGGAAGCTGTGAAGAA\
	AGACAACTCTGTCCATTGGGAGCGCCCTCAGAAACCCAAGGCACCAGTGGGGCATTTTTACGAACCCCAGGCTCCCTCTG\
	CTGAGGTGGAGATGACATCCTATGTGCTCCTCGCTTATCTCACGGCCCAGCCAGCCCCAACCTCGGAGGACCTGACCTCT\
	GCAACCAACATCGTGAAGTGGATCACGAAGCAGCAGAATGCCCAGGGCGGTTTCTCCTCCACCCAGGACA";
    
const DELETE_LINES_TEST_RESULT: &str =
    ">NM_000067.3 Homo sapiens carbonic anhydrase 2 (CA2), transcript variant 1, mRNA\
    ACACAGTGCAGGCGCCCAAGCCGCCGCCGCCAGATCGGTGCCGATTCCTGCCCTGCCCCGACCGCCAGCGCGACCATGTC\
    CCATCACTGGGGGTACGGCAAACACAACGGACCTGAGCACTGGCATAAGGACTTCCCCATTGCCAAGGGAGAGCGCCAGT\
    CCCCTGTTGACATCGACACTCATACAGCCAAGTATGACCCTTCCCTGAAGCCCCTGTCTGTTTCCTATGATCAAGCAACT\
    TCCCTGAGGATCCTCAACAATGGTCATGCTTTCAACGTGGAGTTTGATGACTCTCAGGACAAAGCAGTGCTCAAGGGAGG\
    ACCCCTGGATGGCACTTACAGATTGATTCAGTTTCACTTTCACTGGGGTTCACTTGATGGACAAGGTTCAGAGCATACTG\
    TGGATAAAAAGAAATATGCTGCAGAACTTCACTTGGTTCACTGGAACACCAAATATGGGGATTTTGGGAAAGCTGTGCAG\
    CAACCTGATGGACTGGCCGTTCTAGGTATTTTTTTGAAGGTTGGCAGCGCTAAACCGGGCCTTCAGAAAGTTGTTGATGT\
    GCTGGATTCCATTAAAACAAAGGGCAAGAGTGCTGACTTCACTAACTTCGATCCTCGTGGCCTCCTTCCTGAATCCTTGG\
    ATTACTGGACCTACCCAGGCTCACTGACCACCCCTCCTCTTCTGGAATGTGTGACCTGGATTGTGCTCAAGGAACCCATC\
    AGCGTCAGCAGCGAGCAGGTGTTGAAATTCCGTAAACTTAACTTCAATGGGGAGGGTGAACCCGAAGAACTGATGGTGGA\
    CAACTGGCGCCCAGCTCAGCCACTGAAGAACAGGCAAATCAAAGCTTCCTTCAAATAAGATGGTCCCATAGTCTGTATCC\
    AAATAATGAATCTTCGGGTGTTTCCCTTTAGCTAAGCACAGATCTACCTTGGTGATTTGGACCCTGGTTGCTTTGTGTCT\
    AGTTTTCTAGACCCTTCATCTCTTACTTGATAGACTTACTAATAAAATGTGAAGACTAGACCAATTGTCATGCTTGACAC\
    AACTGCTGTGGCTGGTTGGTGCTTTGTTTATGGTAGTAGTTTTTCTGTAACACAGAATATAGGATAAGAAATAAGAATAA\
    AGTACCTTGACTTTGTTCACAGCATGTAGGGTGATGAGCACTCACAATTGTTGACTAAAATGCTGCTTTTAAAACATAGG\
    AAAGTAGAATGGTTGAGTGCAAATCCATAGCACAAGATAAATTGAGCTAGTTAAGGCAAATCAGGTAAAATAGTCATGAT\
    TCTATGTAATGTAAACCAGAAAAAATAAATGTTCATGATTTCAAGATGTTATATTAAAGAAAAACTTTAAAAATTATTAT\
    ATATTTATAGCAAAGTTATCTTAAATATGAATTCTGTTGTAATTTAATGACTTTTGAATTACAGAGATATAAATGAAGTA\
    TTATCTGTAAAAATTGTTATAATTAGAGTTGTGATACAGAGTATATTTCCATTCAGACAATATATCATAACTTAATAAAT\
    ATTGTATTTTAGATATATTCTCTAATAAAATTCAGAATTCTA";