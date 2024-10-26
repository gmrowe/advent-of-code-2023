(ns aoc2023.day-03
  (:require
   [clojure.string :as str]
   [clojure.java.io :as io]))

(defn- update-schema
  [schema c]
  (cond
    (or (= c \newline) (= c :eof))
    (if-let [digits (:saved-digits schema)]
      (-> schema
          (update :part-nums (fnil conj []) (parse-long (str/join digits)))
          (dissoc :saved-digits))
      schema)

    (Character/isDigit c)
    (-> schema
        (update :data (fnil conj []) [c (count (:part-nums schema))])
        (update :saved-digits (fnil conj []) c))

    :else
    (let [new-schema
          (update schema :data (fnil conj []) c)]
      (if-let [digits (:saved-digits new-schema)]
        (-> new-schema
            (update :part-nums (fnil conj []) (parse-long (str/join digits)))
            (dissoc :saved-digits))
        new-schema))))

;; Parse a well-formed string into a schema.
;; The resulting schema represents the data in the table.
(defn parse-schema
  [s]
  (let [lines (str/split-lines s)
        init {:width (count (first lines)) :height (count lines)}]
    (reduce update-schema init (concat s [:eof]))))

(defn in-bounds?
  [schema row col]
  (and (<= 0 col) (< col (:width schema)) (<= 0 row) (< row (:height schema))))

(defn element-at
  [schema row col]
  (when (in-bounds? schema row col)
    (nth (:data schema) (+ col (* row (:width schema))))))

(defn all-neighbors
  [schema row col]
  (for [dy [-1 0 1]
        dx [-1 0 1]
        :when (and (or (not (zero? dx)) (not (zero? dy)))
                   (in-bounds? schema (+ row dx) (+ col dy)))]
    (element-at schema (+ row dy) (+ col dx))))

(defn adjacent-part-numbers
  [schema row col]
  (let [indexes (->> (all-neighbors schema row col)
                     (filter vector?)
                     (map second))]
    (map (fn [xs] (nth (:part-nums schema) (first xs)))
         (partition-by identity indexes))))

(defn elements
  [schema]
  (for [row (range (:height schema))
        col (range (:width schema))]
    {:row row :col col :element (element-at schema row col)}))

(defn schema-symbol?
  [c]
  (and (not= c \.) (not (vector? c))))

(defn part-01
  [rdr]
  (let [schema (-> rdr
                   slurp
                   parse-schema)]
    (->> (elements schema)
         (filter (fn [{:keys [element]}] (schema-symbol? element)))
         (mapcat (fn [{:keys [row col]}]
                   (adjacent-part-numbers schema row col)))
         (apply +))))

(defn part-02
  [rdr]
  (let [schema (-> rdr
                   slurp
                   parse-schema)]
    (->> (elements schema)
         (filter (fn [{:keys [element]}] (= element \*)))
         (map (fn [{:keys [row col]}] (adjacent-part-numbers schema row col)))
         (filter #(= 2 (count %)))
         (map #(apply * %))
         (apply +))))

(defn -main
  [& args]
  (if-let [input-path (first args)]
    (do (with-open [rdr (io/reader input-path)]
          (printf "[Day-03;Part-01] %s%n" (part-01 rdr)))
        (with-open [rdr (io/reader input-path)]
          (printf "[Day-03;Part-02] %s%n" (part-02 rdr))))
    (println "Usage: clojure -M -m aoc2023.day-03 <input-filename>")))
