(ns aoc2023.day-03-alt-test
  (:require
   [clojure.test :refer [is deftest testing]]
   [clojure.string :as str]
   [aoc2023.day-03-alt :as sut]))

(deftest parse-part-numbers-test
  (testing "Empty input gives empty output"
    (is (empty? (sut/parse-part-numbers ""))))
  (testing "Symbols but no part numbers"
    (is (empty? (sut/parse-part-numbers (str/join \newline
                                                  ["......."
                                                   "......."]))))))

(deftest main-test
  (testing "Everything is gonna be alright"
    (is (= "Hello World!\n" (with-out-str (sut/main))))))
