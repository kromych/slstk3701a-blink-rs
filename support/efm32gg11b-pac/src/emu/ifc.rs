#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `VMONAVDDFALL` writer - Clear VMONAVDDFALL Interrupt Flag"]
pub type VMONAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONAVDDRISE` writer - Clear VMONAVDDRISE Interrupt Flag"]
pub type VMONAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDFALL` writer - Clear VMONALTAVDDFALL Interrupt Flag"]
pub type VMONALTAVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONALTAVDDRISE` writer - Clear VMONALTAVDDRISE Interrupt Flag"]
pub type VMONALTAVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDFALL` writer - Clear VMONDVDDFALL Interrupt Flag"]
pub type VMONDVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONDVDDRISE` writer - Clear VMONDVDDRISE Interrupt Flag"]
pub type VMONDVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0FALL` writer - Clear VMONIO0FALL Interrupt Flag"]
pub type VMONIO0FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO0RISE` writer - Clear VMONIO0RISE Interrupt Flag"]
pub type VMONIO0RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1FALL` writer - Clear VMONIO1FALL Interrupt Flag"]
pub type VMONIO1FALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONIO1RISE` writer - Clear VMONIO1RISE Interrupt Flag"]
pub type VMONIO1RISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VREADY` writer - Clear R5VREADY Interrupt Flag"]
pub type R5VREADY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDFALL` writer - Clear VMONBUVDDFALL Interrupt Flag"]
pub type VMONBUVDDFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMONBUVDDRISE` writer - Clear VMONBUVDDRISE Interrupt Flag"]
pub type VMONBUVDDRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NFETOVERCURRENTLIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLPRUNNING` writer - Clear DCDCLPRUNNING Interrupt Flag"]
pub type DCDCLPRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCLNRUNNING` writer - Clear DCDCLNRUNNING Interrupt Flag"]
pub type DCDCLNRUNNING_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCINBYPASS` writer - Clear DCDCINBYPASS Interrupt Flag"]
pub type DCDCINBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURDY` writer - Clear BURDY Interrupt Flag"]
pub type BURDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5VVSINT` writer - Clear R5VVSINT Interrupt Flag"]
pub type R5VVSINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` writer - Clear EM23WAKEUP Interrupt Flag"]
pub type EM23WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSCALEDONE` writer - Clear VSCALEDONE Interrupt Flag"]
pub type VSCALEDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` writer - Clear TEMP Interrupt Flag"]
pub type TEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` writer - Clear TEMPLOW Interrupt Flag"]
pub type TEMPLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` writer - Clear TEMPHIGH Interrupt Flag"]
pub type TEMPHIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<IFC_SPEC> {
        VMONAVDDFALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<IFC_SPEC> {
        VMONAVDDRISE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<IFC_SPEC> {
        VMONALTAVDDFALL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<IFC_SPEC> {
        VMONALTAVDDRISE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<IFC_SPEC> {
        VMONDVDDFALL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<IFC_SPEC> {
        VMONDVDDRISE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<IFC_SPEC> {
        VMONIO0FALL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<IFC_SPEC> {
        VMONIO0RISE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear VMONIO1FALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio1fall(&mut self) -> VMONIO1FALL_W<IFC_SPEC> {
        VMONIO1FALL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear VMONIO1RISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio1rise(&mut self) -> VMONIO1RISE_W<IFC_SPEC> {
        VMONIO1RISE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear R5VREADY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn r5vready(&mut self) -> R5VREADY_W<IFC_SPEC> {
        R5VREADY_W::new(self, 10)
    }
    #[doc = "Bit 12 - Clear VMONBUVDDFALL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W<IFC_SPEC> {
        VMONBUVDDFALL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear VMONBUVDDRISE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W<IFC_SPEC> {
        VMONBUVDDRISE_W::new(self, 13)
    }
    #[doc = "Bit 16 - Clear PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<IFC_SPEC> {
        PFETOVERCURRENTLIMIT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<IFC_SPEC> {
        NFETOVERCURRENTLIMIT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<IFC_SPEC> {
        DCDCLPRUNNING_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<IFC_SPEC> {
        DCDCLNRUNNING_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<IFC_SPEC> {
        DCDCINBYPASS_W::new(self, 20)
    }
    #[doc = "Bit 22 - Clear BURDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn burdy(&mut self) -> BURDY_W<IFC_SPEC> {
        BURDY_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear R5VVSINT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn r5vvsint(&mut self) -> R5VVSINT_W<IFC_SPEC> {
        R5VVSINT_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<IFC_SPEC> {
        EM23WAKEUP_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear VSCALEDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W<IFC_SPEC> {
        VSCALEDONE_W::new(self, 25)
    }
    #[doc = "Bit 29 - Clear TEMP Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<IFC_SPEC> {
        TEMP_W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear TEMPLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<IFC_SPEC> {
        TEMPLOW_W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<IFC_SPEC> {
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
