(ns aoc2023.day-05
  (:require
   [clojure.string :as str]
   [clojure.java.io :as io]))

(defn split-whitespace
  [s]
  (str/split s #"\s+"))

(defn parse-seeds
  [seeds-str]
  (->> seeds-str
       (re-find #"seeds:\s+(.+)$")
       second
       split-whitespace
       (mapv parse-long)))

(defn parse-mapping-range
  [s]
  (let [[dest-range-start source-range-start range-length]
        (map parse-long (split-whitespace s))]
    {:dest-range-start dest-range-start
     :source-range-start source-range-start
     :range-length range-length}))

(defn parse-mapping
  [s]
  (let [[header & mappings] (str/split-lines s)
        [_ source dest] (re-matches #"(\w+)-to-(\w+)\s+map:" header)]
    {source {:dest dest :mappings (mapv parse-mapping-range mappings)}}))

(defn in-range?
  [num mapping]
  (let [begin (:source-range-start mapping)
        end (+ begin (:range-length mapping))]
    (<= begin num end)))

(defn evaluate-mappings
  [mappings num]
  (let [correct-mapping (some #(when (in-range? num %) %) mappings)]
    (if correct-mapping
      (+ (- num (:source-range-start correct-mapping))
         (:dest-range-start correct-mapping))
      num)))

(defn evaluate
  [conversions src dest value]
  (if (= src dest)
    value
    (let [mp (get conversions src)
          n (evaluate-mappings (:mappings mp) value)]
      (recur conversions (:dest mp) dest n))))

(defn parse-input-01
  [input]
  (let [[seeds-str & mappings-seq] (str/split input #"\n\n")]
    {:seeds (parse-seeds seeds-str)
     :conversions (transduce (map parse-mapping) merge mappings-seq)}))

(defn solve
  [{:keys [seeds conversions]}]
  (transduce (map #(evaluate conversions "seed" "location" %)) min ##Inf seeds))

(defn part-01
  [s]
  (let [[seeds-str & mappings-seq] (str/split s #"\n\n")
        input
        {:seeds (parse-seeds seeds-str)
         :conversions (transduce (map parse-mapping) merge mappings-seq)}]
    (solve input)))

(defn parse-input-02
  [input]
  (let [[seeds-str & mappings-seq] (str/split input #"\n\n")]
    {:seeds (mapcat (fn [[start len]] (range start (+ start len)))
              (partitionv 2 (parse-seeds seeds-str)))
     :conversions (transduce (map parse-mapping) merge mappings-seq)}))

(defn part-02
  [s]
  (let [[seeds-str & mappings-seq] (str/split s #"\n\n")
        input
        {:seeds (mapcat (fn [[start len]] (range start (+ start len)))
                  (partitionv 2 (parse-seeds seeds-str)))
         :conversions (transduce (map parse-mapping) merge mappings-seq)}]
    (solve input)))

(defn -main
  [& args]
  (if-let [input-path (first args)]
    (do (with-open [rdr (io/reader input-path)]
          (printf "[Day-05;Part-01] %s%n" (part-01 (slurp rdr))))
        (with-open [rdr (io/reader input-path)]
          (printf "[Day-05;Part-02] %s%n" (part-02 (slurp rdr)))))
    (println "Usage: clojure -M -m aoc2023.day-05 <input-filename>")))


(comment
  (let [seeds-str "seeds: 79 14 55 13"
        seed-ranges
        (->> seeds-str
             (re-find #"seeds:\s+(.+)$")
             second
             split-whitespace
             (mapv parse-long)
             (partitionv 2)
             vec)

        [start len] (first seed-ranges)]
    (range start (+ start len)))

)

(comment
  (let
    [input
     "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"]
    (part-02 input))



)
