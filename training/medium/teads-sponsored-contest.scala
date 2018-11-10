import scala.collection.mutable.Map
import scala.collection.mutable.Set
import scala.io.StdIn._

object Solution extends App {
  type Relations = Map[Int, Set[Int]]

  val relations: Relations = Map()
  for (i <- 0 until readInt) {
    val Array(x, y) = for(i <- readLine split " ") yield i.toInt
    relations.getOrElseUpdate(x, Set()).add(y)
    relations.getOrElseUpdate(y, Set()).add(x)
  }

  var d = 0
  while (relations.nonEmpty && relations.tail.nonEmpty) {
    d += 1
    val removes: Set[(Int, Int)] = Set()
    for ((k, v) <- relations) {
      if (v.tail.isEmpty) {
        relations.remove(k)
        removes.add((k, v.head))
      }
    }
    for ((l, c) <- removes) {
      relations.get(c).foreach(_.remove(l))
    }
  }

  println(d)
}
