(ns aoc2023.day-01
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]))

(def nums
  {"one" 1
   "two" 2
   "three" 3
   "four" 4
   "five" 5
   "six" 6
   "seven" 7
   "eight" 8
   "nine" 9})

(defn extract-first-text-num
  [s]
  (when-let [text (some #(when (str/starts-with? s %) %) (keys nums))]
    (get nums text)))

(defn extract-first-numeric-num
  [s]
  (when (Character/isDigit (first s)) (Character/getNumericValue (first s))))

(defn extract-first-num
  [s]
  (when (seq s)
    (or (extract-first-text-num s)
        (extract-first-numeric-num s)
        (recur (subs s 1)))))

(defn extract-last-text-num
  [s]
  (when-let [text (some #(when (str/ends-with? s %) %) (keys nums))]
    (get nums text)))

(defn extract-last-numeric-num
  [s]
  (when (Character/isDigit (last s)) (Character/getNumericValue (last s))))

(defn extract-last-num
  [s]
  (when (seq s)
    (or (extract-last-text-num s)
        (extract-last-numeric-num s)
        (recur (subs s 0 (dec (count s)))))))


(defn extract-digits [s] (+ (* 10 (extract-first-num s)) (extract-last-num s)))

(defn extract-numeric-digits
  [s]
  (let [nums (->> s
                  (filter #(Character/isDigit %))
                  (map #(Character/getNumericValue %)))]
    (+ (* 10 (first nums)) (last nums))))

(defn part-01
  [rdr]
  (->> rdr
       line-seq
       (map extract-numeric-digits)
       (reduce + 0)))

(defn part-02
  [rdr]
  (->> rdr
       line-seq
       (map extract-digits)
       (reduce + 0)))

(defn -main
  [& args]
  (if-let [input-path (first args)]
    (do (with-open [rdr (io/reader input-path)]
          (printf "[Day-01;Part-01] %s%n" (part-01 rdr)))
        (with-open [rdr (io/reader input-path)]
          (printf "[Day-01;Part-02] %s%n" (part-02 rdr))))
    (println "Usage: clojure -M -m aoc2023.day-01 <input-filename>")))
<

(comment

  (with-open [rdr (io/reader "resources/day_01_input.txt")]
    (part-01 rdr))

  (with-open [rdr (io/reader "resources/day_01_input.txt")]
    (part-02 rdr))

)
