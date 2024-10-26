(ns aoc2023.day-02-test
  (:require
   [clojure.test :refer [testing is deftest]]
   [aoc2023.day-02 :as sut])
  (:import
   (java.io BufferedReader StringReader)))

(deftest parse-game-test
  (testing "Parsing the game ID"
    (is (= 1
           (:id (sut/parse-game
                 "Game 1:")))))
  (testing "Parsing empty rounds"
    (is (zero? (count (:rounds (:id (sut/parse-game
                                     "Game 1:")))))))
  (testing "Parsing a round with 10 red"
    (is (= {:red 10}
           (-> "Game 42: 10 red"
               sut/parse-game
               :rounds
               first))))
  (testing "Parsing a round with 5 green"
    (is (= {:green 5}
           (-> "Game 420: 5 green"
               sut/parse-game
               :rounds
               first))))
  (testing "Parsing a round with 1 green, 2 blue, and 3 red"
    (is (= {:green 1 :blue 2 :red 3}
           (-> "Game 69: 1 green, 3 red, 2 blue"
               sut/parse-game
               :rounds
               first))))
  (testing "Parsing a game with mulitple rounds"
    (is (= [{:blue 3 :red 4}
            {:red 1 :green 2 :blue 6}
            {:green 2}]
           (-> "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
               sut/parse-game
               :rounds)))))

(deftest game-possible?-test
  (testing "An impossible game given the constraints"
    ;; 12 red cubes, 13 green cubes, and 14 blue cubes
    ;; 8 green, 6 blue, 20 red
    (is (not (sut/round-possible? {:red 20 :blue 6 :green 8}
                                  {:red 12 :green 13 :blue 14}))))
  (testing "A possible game given the constraints"
    (is (sut/round-possible? {:red 8 :green 11 :blue 14}
                             {:red 12 :green 13 :blue 14}))))

(def multi-game-text
  "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")

(deftest part-01-test
  (testing "The implementation of part-01"
    (is (= 8
           (sut/part-01 (BufferedReader.
                         (StringReader.
                          multi-game-text)))))))

(deftest min-requirements-test
  (testing "The min reqirements of a single round"
    (is (= {:red 20 :green 8 :blue 6}
           (sut/min-requirements [{:red 20 :green 8 :blue 6}]))))
  (testing "The min requirements of multiple rounds"
    (is (= {:red 4 :green 2 :blue 6}
           (sut/min-requirements [{:red 4 :blue 3}
                                  {:red 1 :green 2 :blue 6}
                                  {:green 2}])))))

(deftest part-02-test
  (testing "The implementation of part-02"
    (is (= 2286
           (sut/part-02 (BufferedReader.
                         (StringReader.
                          multi-game-text)))))))
