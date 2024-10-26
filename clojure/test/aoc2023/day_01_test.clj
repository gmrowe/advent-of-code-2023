(ns aoc2023.day-01-test
  (:require
   [clojure.test :refer [is are deftest testing]]
   [aoc2023.day-01 :as sut])
  (:import
   (java.io BufferedReader StringReader)))

(deftest problem-01-test
  (testing
    "Various sample inputs can be evaluated"
    (are [x y] (= x (sut/extract-digits y))
     12 "1abc2"
     38 "pqr3stu8vwx"
     15 "a1b2c34d5e"
     77 "treb7uchet"
     29 "two1nine"
     83 "eightwothree"
     13 "abcone2threexyz"
     24 "xtwone3four"
     42 "4nineeightseven2"
     14 "zoneight234"
     76 "7pqrstsixteen"
    )))

(deftest run-test
  (testing
    "Run function takes a newline separated list of inputs sums the outputs"
    (is (= 142
           (sut/part-01 (BufferedReader.
                         (StringReader.
                          "1abc2
pqr3stu8vwx
a1b2c34d5e
treb7uchet")))))))


