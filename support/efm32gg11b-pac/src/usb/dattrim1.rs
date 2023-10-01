#[doc = "Register `DATTRIM1` reader"]
pub type R = crate::R<DATTRIM1_SPEC>;
#[doc = "Register `DATTRIM1` writer"]
pub type W = crate::W<DATTRIM1_SPEC>;
#[doc = "Field `ROUT` reader - Trim for DP and DM Output Impedance for Both FS and LS"]
pub type ROUT_R = crate::FieldReader;
#[doc = "Field `ROUT` writer - Trim for DP and DM Output Impedance for Both FS and LS"]
pub type ROUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `ENDLYPULLUP` reader - Enables Delay of Pull in TX Mode for Both FS and LS"]
pub type ENDLYPULLUP_R = crate::BitReader;
#[doc = "Field `ENDLYPULLUP` writer - Enables Delay of Pull in TX Mode for Both FS and LS"]
pub type ENDLYPULLUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLYPULLUPFS` reader - Trim for Rising Crossover Voltage in FS"]
pub type DLYPULLUPFS_R = crate::FieldReader;
#[doc = "Field `DLYPULLUPFS` writer - Trim for Rising Crossover Voltage in FS"]
pub type DLYPULLUPFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VCRSFS` reader - Trim for Falling Crossover Voltage in FS"]
pub type VCRSFS_R = crate::FieldReader;
#[doc = "Field `VCRSFS` writer - Trim for Falling Crossover Voltage in FS"]
pub type VCRSFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TFDMFS` reader - Trim for DM Fall Time in FS"]
pub type TFDMFS_R = crate::FieldReader;
#[doc = "Field `TFDMFS` writer - Trim for DM Fall Time in FS"]
pub type TFDMFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TRDMFS` reader - Trim for DM Rise Time in FS"]
pub type TRDMFS_R = crate::FieldReader;
#[doc = "Field `TRDMFS` writer - Trim for DM Rise Time in FS"]
pub type TRDMFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TFDPFS` reader - Trim for DP Fall Time in FS"]
pub type TFDPFS_R = crate::FieldReader;
#[doc = "Field `TFDPFS` writer - Trim for DP Fall Time in FS"]
pub type TFDPFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TRDPFS` reader - Trim for DP Rise Time in FS"]
pub type TRDPFS_R = crate::FieldReader;
#[doc = "Field `TRDPFS` writer - Trim for DP Rise Time in FS"]
pub type TRDPFS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&self) -> ROUT_R {
        ROUT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&self) -> ENDLYPULLUP_R {
        ENDLYPULLUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&self) -> DLYPULLUPFS_R {
        DLYPULLUPFS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&self) -> VCRSFS_R {
        VCRSFS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&self) -> TFDMFS_R {
        TFDMFS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&self) -> TRDMFS_R {
        TRDMFS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&self) -> TFDPFS_R {
        TFDPFS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&self) -> TRDPFS_R {
        TRDPFS_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    #[must_use]
    pub fn rout(&mut self) -> ROUT_W<DATTRIM1_SPEC, 0> {
        ROUT_W::new(self)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    #[must_use]
    pub fn endlypullup(&mut self) -> ENDLYPULLUP_W<DATTRIM1_SPEC, 7> {
        ENDLYPULLUP_W::new(self)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    #[must_use]
    pub fn dlypullupfs(&mut self) -> DLYPULLUPFS_W<DATTRIM1_SPEC, 8> {
        DLYPULLUPFS_W::new(self)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    #[must_use]
    pub fn vcrsfs(&mut self) -> VCRSFS_W<DATTRIM1_SPEC, 10> {
        VCRSFS_W::new(self)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    #[must_use]
    pub fn tfdmfs(&mut self) -> TFDMFS_W<DATTRIM1_SPEC, 12> {
        TFDMFS_W::new(self)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    #[must_use]
    pub fn trdmfs(&mut self) -> TRDMFS_W<DATTRIM1_SPEC, 14> {
        TRDMFS_W::new(self)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    #[must_use]
    pub fn tfdpfs(&mut self) -> TFDPFS_W<DATTRIM1_SPEC, 16> {
        TFDPFS_W::new(self)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    #[must_use]
    pub fn trdpfs(&mut self) -> TRDPFS_W<DATTRIM1_SPEC, 18> {
        TRDPFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data TRIM 1 Values for USB DP and DM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dattrim1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dattrim1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATTRIM1_SPEC;
impl crate::RegisterSpec for DATTRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dattrim1::R`](R) reader structure"]
impl crate::Readable for DATTRIM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dattrim1::W`](W) writer structure"]
impl crate::Writable for DATTRIM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATTRIM1 to value 0x24"]
impl crate::Resettable for DATTRIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x24;
}
