import scala.io.StdIn.readInt

object TheDescent extends App {
  case class Highest(index: Int, height: Int)

  while(true) {
    println((0 until 8).foldLeft(Highest(-1, Int.MinValue))((highest, idx) => {
      val height = readInt
      if (height > highest.height) Highest(idx, height) else highest
    }).index)
  }
}
