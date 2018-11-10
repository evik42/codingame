import scala.io.StdIn._

object Solution extends App {
  val l = readInt
  val h = readInt
  val t = readLine
  (0 until h)
    .map(_ => t.toCharArray.map(transformCharacter(readLine)).mkString)
    .foreach(println)

  def transformCharacter(row: String)(c: Char): String = {
    val pos = c.toUpper - 'A'
    val start = (if (pos < 0 || pos > 25) 26 else pos) * l
    row.substring(start, start + l)
  }
}
