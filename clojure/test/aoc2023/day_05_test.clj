es
(ns aoc2023.day-05-test
  (:require
   [clojure.test :refer [deftest testing is]]
   [aoc2023.day-05 :as sut]))

(defn rng
  [start length]
  {:range-start start
   :range-length length})

(deftest union-test
  (testing "The union of two identical ranges is the range"
    (is (= (rng 0 10) (sut/union (rng 0 10) (rng 0 10)))))
  (testing "The union of two distinct ranges is nil"
    (is (nil? (sut/union (rng 0 10) (rng 100 10)))))
  (testing "The union of a range and a subrange is the subrange"
    (is (= (rng 1 2)
           (sut/union (rng 1 2)   ;; 1 - 2
                      (rng 0 5)   ;; 0 - 4
           ))))
  (testing "The union of partially overlapping ranges"
    (is (= (rng 4 2)
           (sut/union (rng 1 5) ;; 1 2 3 4 5
                      (rng 4 3) ;; . . . 4 5 6
           ))))
  (testing "The union operation is communative"
    (is (= (sut/union (rng 1 5) (rng 4 3))
           (sut/union (rng 4 3) (rng 1 5))))))

(deftest difference-test
  (testing "If ranges do not overlap at all, the first range is returned"
    (is (= [(rng 0 10)] (sut/difference (rng 0 10) (rng 100 10)))))
  (testing "If ranges are equal, the difference is nil"
    (is (nil? (sut/difference (rng 0 5) (rng 0 5)))))
  (testing "If first range is a subrange of second range, the difference is nil"
    (is (nil? (sut/difference (rng 1 10) (rng 0 100)))))
  (testing
    "If second range ends before first range, the difference is not nil"
    (is (= [(rng 10 90)] (sut/difference (rng 0 100) (rng 0 10)))))
  (testing
    "If second range starts after first range the difference is not nil"
    (is (= [(rng 0 10)] (sut/difference (rng 0 100) (rng 10 90)))))
  (testing
    "If second range starts after first range and ends before first range, two ranges are returned"
    (is (= [(rng 0 10) (rng 90 10)] (sut/difference (rng 0 100) (rng 10 80))))))
