
import { Router } from "express";
import { SUPERADMIN_JWT_PASSWORD } from "../../../config";
import { getToken } from "../../../utils/totp";
import { SignInSchema, SignInVerifySchema } from "@repo/common/types";
import { adminSignIn, adminSignInVerify } from "../common";

const router: Router = Router();

router.post("/signin", async (req, res) => {
    const parsedData = SignInSchema.safeParse(req.body);
    if (!parsedData.success) {
        res.status(400).json({
            message: "Invalid data"
        })
        return
    }
    const number = parsedData.data.number;
    const totp = getToken(number, "ADMIN_AUTH");
    await adminSignIn(req, res, number, totp);

});

router.post("/signin/verify", async (req, res) => {
    const parsedData = SignInVerifySchema.safeParse(req.body);
    if (!parsedData.success) {
        res.status(400).json({
            message: "Invalid data"
        })
        return
    }

    const number = parsedData.data.number;
    const otp = parsedData.data.totp;
    await adminSignInVerify(req, res, number, otp, SUPERADMIN_JWT_PASSWORD);

});

export default router;
