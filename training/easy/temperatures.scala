import scala.io.StdIn.readLine
import math._

object Temperatures extends App {
  val n = readLine
  val res = if (n == "0") {
    n
  } else {
    readLine
      .split(" ")
      .map(_.toInt)
      .reduce((a, b) => { if (abs(a) == abs(b)) max(a, b) else if (abs(a) < abs(b)) a else b })
      .toString
  }
  println(res)
}
