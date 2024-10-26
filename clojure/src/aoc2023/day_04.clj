(ns aoc2023.day-04
  (:require
   [clojure.string :as str]
   [clojure.set :as set]
   [clojure.java.io :as io]))

(defn split-whitespace
  [s]
  (-> s
      str/trim
      (str/split #"\s+")))

;; Regex to parse a line in the format:
;; Card 51: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
;; The parse result (from for instance calling re-matches):
;; => ["Card 51: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
;;     "51"
;;     "41 48 83 86 17"
;;     "83 86  6 31 17  9 48 53"]
(def hand-pattern
  (let [card-keyword #"Card\s+"
        card-id #"(\d+):\s+"
        winning-numbers #"((?:\d+\s*)+)"
        bar-separator #"\s+\|\s+"
        player-numbers #"((?:\d+\s*)+)"]
    (re-pattern
      (str card-keyword card-id winning-numbers bar-separator player-numbers))))

(defn parse-hand
  [s]
  (let [[_ id winning-numbers player-numbers] (re-matches hand-pattern s)]
    {:id (parse-long id)
     :winning (mapv parse-long (split-whitespace winning-numbers))
     :player (mapv parse-long (split-whitespace player-numbers))}))

(defn matching-nums
  [hand]
  (set/intersection (into #{} (:winning hand)) (into #{} (:player hand))))

(defn score
  [hand]
  (let [match-count (count (matching-nums hand))]
    (nth (cons 0 (iterate #(* 2 %) 1)) match-count)))

(defn part-01
  [rdr]
  (transduce (map (fn [line]
                    (-> line
                        parse-hand
                        score)))
             +
             (line-seq rdr)))

(defn part-02
  [rdr]
  (let [hands (mapv parse-hand (line-seq rdr))
        update-counts
        (fn [counts hand]
          (let [id (:id hand)
                copies (nth counts (dec id))
                match-count (count (matching-nums hand))
                indexes (range id (+ id match-count))]
            (reduce (fn [cs i] (update cs i + copies)) counts indexes)))

        counts (reduce update-counts (vec (repeat (count hands) 1)) hands)]
    (apply + counts)))

(defn -main
  [& args]
  (if-let [input-path (first args)]
    (do (with-open [rdr (io/reader input-path)]
          (printf "[Day-04;Part-01] %s%n" (part-01 rdr)))
        (with-open [rdr (io/reader input-path)]
          (printf "[Day-03;Part-02] %s%n" (part-02 rdr))))
    (println "Usage: clojure -M -m aoc2023.day-03 <input-filename>")))


(comment

  (import (java.io StringReader BufferedReader))
  (let [s ["Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
           "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
           "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
           "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"
           "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"
           "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"]
        lines (str/join \newline s)]
    (part-02 (BufferedReader. (StringReader. lines))))

)
