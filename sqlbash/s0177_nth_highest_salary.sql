# Write your MySQL query statement below
CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  SET N = N-1;
  RETURN (
    SELECT
      IFNULL(
        (SELECT DISTINCT Salary
          FROM Employee
          ORDER BY Salary DESC
          LIMIT 1 OFFSET N),
      NULL) AS SecondHighestSalary
  );
END

