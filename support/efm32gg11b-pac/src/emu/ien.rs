#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `VMONAVDDFALL` reader - VMONAVDDFALL Interrupt Enable"]
pub type VMONAVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONAVDDFALL` writer - VMONAVDDFALL Interrupt Enable"]
pub type VMONAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` reader - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONAVDDRISE` writer - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` reader - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONALTAVDDFALL` writer - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` reader - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONALTAVDDRISE` writer - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` reader - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONDVDDFALL` writer - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` reader - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONDVDDRISE` writer - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` reader - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_R = crate::BitReader;
#[doc = "Field `VMONIO0FALL` writer - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` reader - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_R = crate::BitReader;
#[doc = "Field `VMONIO0RISE` writer - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1FALL` reader - VMONIO1FALL Interrupt Enable"]
pub type VMONIO1FALL_R = crate::BitReader;
#[doc = "Field `VMONIO1FALL` writer - VMONIO1FALL Interrupt Enable"]
pub type VMONIO1FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1RISE` reader - VMONIO1RISE Interrupt Enable"]
pub type VMONIO1RISE_R = crate::BitReader;
#[doc = "Field `VMONIO1RISE` writer - VMONIO1RISE Interrupt Enable"]
pub type VMONIO1RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VREADY` reader - R5VREADY Interrupt Enable"]
pub type R5VREADY_R = crate::BitReader;
#[doc = "Field `R5VREADY` writer - R5VREADY Interrupt Enable"]
pub type R5VREADY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDFALL` reader - VMONBUVDDFALL Interrupt Enable"]
pub type VMONBUVDDFALL_R = crate::BitReader;
#[doc = "Field `VMONBUVDDFALL` writer - VMONBUVDDFALL Interrupt Enable"]
pub type VMONBUVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDRISE` reader - VMONBUVDDRISE Interrupt Enable"]
pub type VMONBUVDDRISE_R = crate::BitReader;
#[doc = "Field `VMONBUVDDRISE` writer - VMONBUVDDRISE Interrupt Enable"]
pub type VMONBUVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_R = crate::BitReader;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_R = crate::BitReader;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` reader - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_R = crate::BitReader;
#[doc = "Field `DCDCLPRUNNING` writer - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` reader - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_R = crate::BitReader;
#[doc = "Field `DCDCLNRUNNING` writer - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` reader - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_R = crate::BitReader;
#[doc = "Field `DCDCINBYPASS` writer - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURDY` reader - BURDY Interrupt Enable"]
pub type BURDY_R = crate::BitReader;
#[doc = "Field `BURDY` writer - BURDY Interrupt Enable"]
pub type BURDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VVSINT` reader - R5VVSINT Interrupt Enable"]
pub type R5VVSINT_R = crate::BitReader;
#[doc = "Field `R5VVSINT` writer - R5VVSINT Interrupt Enable"]
pub type R5VVSINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` reader - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_R = crate::BitReader;
#[doc = "Field `EM23WAKEUP` writer - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSCALEDONE` reader - VSCALEDONE Interrupt Enable"]
pub type VSCALEDONE_R = crate::BitReader;
#[doc = "Field `VSCALEDONE` writer - VSCALEDONE Interrupt Enable"]
pub type VSCALEDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` reader - TEMP Interrupt Enable"]
pub type TEMP_R = crate::BitReader;
#[doc = "Field `TEMP` writer - TEMP Interrupt Enable"]
pub type TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` reader - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_R = crate::BitReader;
#[doc = "Field `TEMPLOW` writer - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` reader - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_R = crate::BitReader;
#[doc = "Field `TEMPHIGH` writer - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VMONAVDDFALL_R {
        VMONAVDDFALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VMONAVDDRISE_R {
        VMONAVDDRISE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VMONALTAVDDFALL_R {
        VMONALTAVDDFALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VMONALTAVDDRISE_R {
        VMONALTAVDDRISE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VMONDVDDFALL_R {
        VMONDVDDFALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VMONDVDDRISE_R {
        VMONDVDDRISE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> VMONIO0FALL_R {
        VMONIO0FALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> VMONIO0RISE_R {
        VMONIO0RISE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VMONIO1FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1fall(&self) -> VMONIO1FALL_R {
        VMONIO1FALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VMONIO1RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio1rise(&self) -> VMONIO1RISE_R {
        VMONIO1RISE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - R5VREADY Interrupt Enable"]
    #[inline(always)]
    pub fn r5vready(&self) -> R5VREADY_R {
        R5VREADY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&self) -> VMONBUVDDFALL_R {
        VMONBUVDDFALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&self) -> VMONBUVDDRISE_R {
        VMONBUVDDRISE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PFETOVERCURRENTLIMIT_R {
        PFETOVERCURRENTLIMIT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NFETOVERCURRENTLIMIT_R {
        NFETOVERCURRENTLIMIT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DCDCLPRUNNING_R {
        DCDCLPRUNNING_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DCDCLNRUNNING_R {
        DCDCLNRUNNING_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DCDCINBYPASS_R {
        DCDCINBYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - R5VVSINT Interrupt Enable"]
    #[inline(always)]
    pub fn r5vvsint(&self) -> R5VVSINT_R {
        R5VVSINT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<IEN_SPEC> {
        VMONAVDDFALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<IEN_SPEC> {
        VMONAVDDRISE_W::new(self, 1)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<IEN_SPEC> {
        VMONALTAVDDFALL_W::new(self, 2)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<IEN_SPEC> {
        VMONALTAVDDRISE_W::new(self, 3)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<IEN_SPEC> {
        VMONDVDDFALL_W::new(self, 4)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<IEN_SPEC> {
        VMONDVDDRISE_W::new(self, 5)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<IEN_SPEC> {
        VMONIO0FALL_W::new(self, 6)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<IEN_SPEC> {
        VMONIO0RISE_W::new(self, 7)
    }
    #[doc = "Bit 8 - VMONIO1FALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio1fall(&mut self) -> VMONIO1FALL_W<IEN_SPEC> {
        VMONIO1FALL_W::new(self, 8)
    }
    #[doc = "Bit 9 - VMONIO1RISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio1rise(&mut self) -> VMONIO1RISE_W<IEN_SPEC> {
        VMONIO1RISE_W::new(self, 9)
    }
    #[doc = "Bit 10 - R5VREADY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn r5vready(&mut self) -> R5VREADY_W<IEN_SPEC> {
        R5VREADY_W::new(self, 10)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W<IEN_SPEC> {
        VMONBUVDDFALL_W::new(self, 12)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W<IEN_SPEC> {
        VMONBUVDDRISE_W::new(self, 13)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<IEN_SPEC> {
        PFETOVERCURRENTLIMIT_W::new(self, 16)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<IEN_SPEC> {
        NFETOVERCURRENTLIMIT_W::new(self, 17)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<IEN_SPEC> {
        DCDCLPRUNNING_W::new(self, 18)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<IEN_SPEC> {
        DCDCLNRUNNING_W::new(self, 19)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<IEN_SPEC> {
        DCDCINBYPASS_W::new(self, 20)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn burdy(&mut self) -> BURDY_W<IEN_SPEC> {
        BURDY_W::new(self, 22)
    }
    #[doc = "Bit 23 - R5VVSINT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn r5vvsint(&mut self) -> R5VVSINT_W<IEN_SPEC> {
        R5VVSINT_W::new(self, 23)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<IEN_SPEC> {
        EM23WAKEUP_W::new(self, 24)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W<IEN_SPEC> {
        VSCALEDONE_W::new(self, 25)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<IEN_SPEC> {
        TEMP_W::new(self, 29)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<IEN_SPEC> {
        TEMPLOW_W::new(self, 30)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<IEN_SPEC> {
        TEMPHIGH_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
