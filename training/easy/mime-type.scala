import scala.io.StdIn._
import scala.collection.mutable.HashMap

object Solution extends App {
  val n = readInt
  val q = readInt
  val types: HashMap[String, String] = HashMap()
  for(_ <- 0 until n) {
    val Array(ext, mt) = readLine split " "
    types.put(ext.toUpperCase, mt)
  }
  for(_ <- 0 until q) {
    val fname = readLine
    val idx = fname.lastIndexOf('.')
    println(types.getOrElse(if (idx > -1) { fname.substring(idx + 1).toUpperCase } else { "" }, "UNKNOWN"))
  }
}
