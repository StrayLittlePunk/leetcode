# Write your MySQL query statement below
SELECT Score, (SELECT count(*) FROM 
  (SELECT DISTINCT Score s FROM Scores) tmp WHERE s >= Score) `Rank` 
FROM Scores ORDER BY Score DESC;

SELECT
  Score,
  @rank := @rank + (@prev <> (@prev := Score)) `Rank`
FROM
  Scores,
  (SELECT @rank := 0, @prev := -1) init
ORDER BY Score desc
