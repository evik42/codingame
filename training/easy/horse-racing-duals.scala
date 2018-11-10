import scala.io.StdIn.readInt

object Solution extends App {
  def folder(h:Int, state:(Int, Int)): (Int, Int) = {
    val d = state._1 - h
    (h, math.min(d, state._2))
  }

  val res = (0 until readInt)
    .map(_ => readInt)
    .sorted
    .sliding(2)
    .map(s => s(1) - s(0))
    .min

  println(res)
}
