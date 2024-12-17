#!/usr/bin/python3
import unittest
import nasparse


class TestNasparse(unittest.TestCase):
    imsi_sent_msg = '07412208391185184409309005f0700000100030023ed031d127298080211001000010810600000000830600000000000d00000300ff0003130184000a000005000010005c0a009011034f18a6f15d0103c1000000000000'
    sec_imsi_sent_msg = '1727db4b7c0207412208391185184409309005f0700000100030023ed031d127298080211001000010810600000000830600000000000d00000300ff0003130184000a000005000010005c0a009011034f18a6f15d0103c1'
    non_nas_msg = 'deadbeefcafe'
    other_nas_msg = '074413780004023fd121'
    other_nas_mt_msg = "023fd12100000000000000000000000000000000000000000000000000000000"
    ciphered_nas_msg = "27ed6146bd0162a5d62d62e1ce501720dc8bd84f1167fd"

    def run_heur(self, msg):
        buf = nasparse.parse_nas_message(msg)
        return nasparse.heur_ue_imsi_sent(buf)[0]

    def test_imsi_sent(self):
        self.assertEqual(self.run_heur(self.imsi_sent_msg), True, "imsi_sent_msg should trigger heuristic")

    def test_sec_imsi_sent(self):
        self.assertEqual(self.run_heur(self.imsi_sent_msg), True, "sec_imsi_sent_msg should trigger heuristic")

    def test_non_nas_msg(self):
        with self.assertRaises(TypeError):
            self.run_heur(self.non_nas_msg)

    def test_other_nas(self):
        self.assertEqual(self.run_heur(self.other_nas_msg), False, "other_nas_msg should not trigger heuristic")

    def test_other_nas_mt(self):
        self.assertEqual(self.run_heur(self.other_nas_mt_msg), False, "other_nas_mt_msg should not trigger heuristic")

    def test_ciphered_nas(self):
        self.assertEqual(self.run_heur(self.ciphered_nas_msg), False, "ciphered_nas_msg should not trigger heuristic")

if __name__ == '__main__':
    unittest.main()
