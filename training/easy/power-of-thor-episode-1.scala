import scala.annotation.tailrec
import scala.io.StdIn._

object Player extends App {
  val Array(lightx, lighty, initialtx, initialty) = for(i <- readLine split " ") yield i.toInt

  loop(initialtx, initialty)

  @tailrec
  def loop(x: Int, y: Int): Unit = {
    val newy = if (y > lighty) {
      print("N")
      y - 1
    } else if (y < lighty) {
      print("S")
      y + 1
    } else {
      y
    }
    val newx = if (x > lightx) {
      print("W")
      x - 1
    } else if (x < lightx) {
      print("E")
      x + 1
    } else {
      x
    }
    println("")
    loop(newx, newy)
  }
}
