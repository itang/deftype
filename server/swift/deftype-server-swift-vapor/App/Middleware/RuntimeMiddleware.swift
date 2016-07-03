import Vapor
import Foundation

class RuntimeMiddleware: Middleware {

    func respond(to request: Request, chainingTo chain: Responder) throws -> Response {
        let date = NSDate()
        let start = date.timeIntervalSince1970

        let response = try chain.respond(to: request)

        let end = date.timeIntervalSince1970

        response.headers["XRuntime"] = "\(end - start)"

        return response
    }
}
