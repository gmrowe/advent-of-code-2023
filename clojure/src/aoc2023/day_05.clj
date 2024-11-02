(ns aoc2023.day-05
  (:require
   [clojure.string :as str]
   [clojure.java.io :as io]))

(defn split-whitespace [s] (str/split s #"\s+"))

(defn rng
  [start length]
  {:range-start start
   :range-length length})

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
     :range-start source-range-start
     :range-length range-length}))

(defn parse-mapping
  [s]
  (let [[header & mappings] (str/split-lines s)
        [_ source dest] (re-matches #"(\w+)-to-(\w+)\s+map:" header)]
    {source {:dest dest :mappings (mapv parse-mapping-range mappings)}}))

(defn in-range?
  [num mapping]
  (let [begin (:range-start mapping)
        end (+ begin (:range-length mapping))]
    (<= begin num end)))

(defn evaluate-mappings
  [mappings num]
  (let [correct-mapping (some #(when (in-range? num %) %) mappings)]
    (if correct-mapping
      (+ (- num (:range-start correct-mapping))
         (:dest-range-start correct-mapping))
      num)))

(defn evaluate
  [conversions src dest value]
  (if (= src dest)
    value
    (let [mp (get conversions src)
          n (evaluate-mappings (:mappings mp) value)]
      (recur conversions (:dest mp) dest n))))

(defn union
  [rng-a rng-b]
  (let [a-start (:range-start rng-a)
        a-end (+ a-start (:range-length rng-a))
        b-start (:range-start rng-b)
        b-end (+ b-start (:range-length rng-b))
        start (max a-start b-start)
        end (min a-end b-end)
        length (- end start)]
    (when (pos? length) (rng start length))))

(defn difference
  [rng-a rng-b]
  (let [a-start (:range-start rng-a)
        a-end (+ a-start (:range-length rng-a))
        b-start (:range-start rng-b)
        b-end (+ b-start (:range-length rng-b))
        overlap? (and (<= b-start a-end) (>= b-end a-start))]
    (if overlap?
      (filterv (fn [{:keys [range-length]}] (pos? range-length))
               [(rng a-start (- b-start a-start)) (rng b-end (- a-end b-end))])
      [rng-a])))

(defn convert-ranges
  [mappings ranges]
  (loop [in ranges
         out []]
    (if-let [rng (first in)]
      (let [mp (some #(when (union rng %) %) mappings)
            result (if mp
                     (update (union rng mp)
                             :range-start
                             +
                             (:dest-range-start mp)
                             (- (:range-start mp)))
                     rng)
            diffs (when mp (difference rng mp))]
        (recur (concat diffs (next in)) (conj out result)))
      out)))

(defn evaluate-ranges
  [conversions src target-dest ranges]
  (if (= src target-dest)
    ranges
    (let [{:keys [mappings dest]} (get conversions src)
          rs (convert-ranges mappings ranges)]
      (recur conversions dest target-dest rs))))

(defn part-01
  [s]
  (let [[seeds-str & mappings-seq] (str/split s #"\n\n")
        seeds (parse-seeds seeds-str)
        conversions (transduce (map parse-mapping) merge mappings-seq)]
    (transduce (map #(evaluate conversions "seed" "location" %))
               min
               ##Inf
               seeds)))

(defn part-02
  [input]
  (let [[seeds-str & mappings-seq] (str/split input #"\n\n")

        seeds (->> seeds-str
                   parse-seeds
                   (partition 2)
                   (map (fn [[start length]] [(rng start length)])))


        conversions
        (transduce (map parse-mapping) merge mappings-seq)]
    (transduce (comp (mapcat
                      (fn [rng]
                        (evaluate-ranges conversions "seed" "location" rng)))
                     (map :range-start))
               min
               ##Inf
               seeds)))

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

