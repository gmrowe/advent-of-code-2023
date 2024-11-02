(ns aoc2023.day-03-test
  (:require
   [clojure.test :refer [is deftest testing]]
   [clojure.string :as str]
   [aoc2023.day-03 :as sut])
  (:import
   (java.io BufferedReader StringReader)))

(defn unlines [ss] (str/join \newline ss))

(defn str-rdr [s] (BufferedReader. (StringReader. s)))

#_(deftest extract-part-numbers-test
    (testing "Empty input returns no part numbers"
      (is (empty? (sut/extract-part-numbers ""))))
    (testing "Number not adjacent to a symbol is not a part number"
      (is (empty? (sut/extract-part-numbers "..123.."))))
    (testing "Single digit to the left of a symbol"
      (is (= [3] (sut/extract-part-numbers "..3*..."))))
    (testing "Single digit to the right of a symbol"
      (is (= [6] (sut/extract-part-numbers "...$6..."))))
    (testing "Single digit above symbol"
      (is (= [9]
             (sut/extract-part-numbers (unlines ["...9..."
                                                 "...#..."])))))
    (testing "Single digit below symbol"
      (is (= [5]
             (sut/extract-part-numbers (unlines ["...?..."
                                                 "...5..."])))))
    (testing "Single digit above-left of symbol"
      (is (= [3]
             (sut/extract-part-numbers (unlines ["..3...."
                                                 "...&..."])))))
    (testing "Single digit above-right of symbol"
      (is (= [6]
             (sut/extract-part-numbers (unlines ["....6.."
                                                 "...&..."])))))
    (testing "Single digit below-left of symbol"
      (is (= [2]
             (sut/extract-part-numbers (unlines ["...*..."
                                                 "..2...."])))))
    (testing "Single digit below-right of symbol"
      (is (= [9]
             (sut/extract-part-numbers (unlines ["...(..."
                                                 "....9.."])))))
    (testing "Multi-digit number to the left of symbol"
      (is (= [123]
             (sut/extract-part-numbers "..123*."))))
    (testing "Multi-digit number to the right of symbol"
      (is (= [123] (sut/extract-part-numbers "..%123.."))))
    (testing "Multi-digit number over more than one neighbor"
      (is (= [456]
             (sut/extract-part-numbers (unlines ["...(..."
                                                 "...456."])))))
    (testing "A part number cannot span lines"
      (is (= [678]
             (sut/extract-part-numbers (unlines [".(..3"
                                                 "678.."]))))))

(deftest part-01-test
  (testing "Part-1 with sample input"
    (is
     (=
      4361
      (sut/part-01
       (str-rdr
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."))))))
