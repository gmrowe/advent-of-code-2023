(ns aoc2023.day-02
  (:require
   [clojure.string :as str]
   [clojure.java.io :as io]))

(defn strip-prefix
  [s prefix]
  (if (str/starts-with? s prefix) (subs s (count prefix)) s))

(defn parse-round
  [round]
  (let [colors (str/split round #",")]
    (->> colors
         (map (fn [s]
                (let [[count color] (-> s
                                        str/trim
                                        (str/split #"\s+"))]
                  {(keyword color) (parse-long count)})))
         (apply merge))))

(defn parse-game-id
  [id]
  (-> id
      (strip-prefix "Game ")
      parse-long))

;; Game string looks like:
;; "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
;; Game <id>: <game-data><; ..more data>
;; id = long
;; game-data = <n-color> <color>
;; where allowable colors are red, green and blue
;; if no entry for a particular color, it is assumed to be 0
(defn parse-game
  [s]
  (let [[id rounds] (str/split s #":" 2)]
    {:id (parse-game-id id) :rounds (map parse-round (str/split rounds #";"))}))

(defn round-possible?
  [round constraint]
  (->> (merge-with <= round constraint)
       vals
       (every? identity)))

(defn part-01
  [rdr]
  (let [constraint {:red 12 :green 13 :blue 14}]
    (->> rdr
         line-seq
         (map parse-game)
         (filter (fn [game]
                   (every? #(round-possible? % constraint) (:rounds game))))
         (map :id)
         (reduce + 0))))



(defn min-requirements
  [rounds]
  (reduce (fn [res round] (merge-with max res round)) rounds))

(defn power [{:keys [red blue green]}] (* red blue green))

(def part-02-tranducer
  (comp (map parse-game) (map :rounds) (map min-requirements) (map power)))

(defn part-02 [rdr] (transduce part-02-tranducer + (line-seq rdr)))

(defn -main
  [& args]
  (if-let [input-path (first args)]
    (do (with-open [rdr (io/reader input-path)]
          (printf "[Day-02;Part-01] %s%n" (part-01 rdr)))
        (with-open [rdr (io/reader input-path)]
          (printf "[Day-01;Part-02] %s%n" (part-02 rdr))))
    (println "Usage: clojure -M -m aoc2023.day-02 <input-filename>")))
