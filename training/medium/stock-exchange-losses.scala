import scala.io.StdIn._

object StockExchangeLosses extends App {
  val n = readInt
  val loss = (readLine split " ")
    .map(_.toInt)
    .foldLeft((0, 0))((s, v) => (math.max(v, s._1), math.min(v - s._1, s._2)))
    ._2
  println(loss)
}
