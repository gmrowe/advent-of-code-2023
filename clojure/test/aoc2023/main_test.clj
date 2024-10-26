(ns aoc2023.main-test
  (:require
   [clojure.test :refer [is are deftest testing]]
   [aoc2023.main :as sut]))

(deftest main-test
  (testing "Main is \"Hello World!\""
    (is (= "Hello World!" (with-out-str (sut/-main))))))


