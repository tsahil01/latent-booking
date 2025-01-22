import { client } from "@repo/db/client";
import { sendMessage } from "../../../utils/twilio";
import { Request, Response } from "express";
import { verifyToken } from "../../../utils/totp";
import jwt from "jsonwebtoken";

export async function adminSignIn(req: Request, res: Response, number: string, totp: string) {
    try {
        await client.admin.findFirstOrThrow({
            where: {
                number
            }
        });

        if (process.env.NODE_ENV === "production") {
            console.log("inside send message")
            try {
                await sendMessage(`Your admin otp for logging into latent is ${totp}`, number)
            } catch (e) {
                res.status(500).json({
                    message: "Could not send otp"
                })
                return
            }
        }

        res.json({
            message: "Otp sent"
        })
    } catch (e) {
        res.status(411).json({
            message: "User invalid"
        })
    }
}

export async function adminSignInVerify(req: Request, res: Response, number: string, otp: string, jwtPassword: string) {
    if (process.env.NODE_ENV === "production" && !verifyToken(number, "ADMIN_AUTH", otp)) {
        res.json({
            message: "Invalid token"
        })
        return
    }

    const user = await client.user.findFirstOrThrow({
        where: {
            number
        }
    })

    const token = jwt.sign({
        userId: user.id
    }, jwtPassword)

    res.json({
        token
    })
}