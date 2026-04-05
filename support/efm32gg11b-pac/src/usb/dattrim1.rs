#[doc = "Register `DATTRIM1` reader"]
pub type R = crate::R<Dattrim1Spec>;
#[doc = "Register `DATTRIM1` writer"]
pub type W = crate::W<Dattrim1Spec>;
#[doc = "Field `ROUT` reader - Trim for DP and DM Output Impedance for Both FS and LS"]
pub type RoutR = crate::FieldReader;
#[doc = "Field `ROUT` writer - Trim for DP and DM Output Impedance for Both FS and LS"]
pub type RoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ENDLYPULLUP` reader - Enables Delay of Pull in TX Mode for Both FS and LS"]
pub type EndlypullupR = crate::BitReader;
#[doc = "Field `ENDLYPULLUP` writer - Enables Delay of Pull in TX Mode for Both FS and LS"]
pub type EndlypullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYPULLUPFS` reader - Trim for Rising Crossover Voltage in FS"]
pub type DlypullupfsR = crate::FieldReader;
#[doc = "Field `DLYPULLUPFS` writer - Trim for Rising Crossover Voltage in FS"]
pub type DlypullupfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VCRSFS` reader - Trim for Falling Crossover Voltage in FS"]
pub type VcrsfsR = crate::FieldReader;
#[doc = "Field `VCRSFS` writer - Trim for Falling Crossover Voltage in FS"]
pub type VcrsfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TFDMFS` reader - Trim for DM Fall Time in FS"]
pub type TfdmfsR = crate::FieldReader;
#[doc = "Field `TFDMFS` writer - Trim for DM Fall Time in FS"]
pub type TfdmfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRDMFS` reader - Trim for DM Rise Time in FS"]
pub type TrdmfsR = crate::FieldReader;
#[doc = "Field `TRDMFS` writer - Trim for DM Rise Time in FS"]
pub type TrdmfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TFDPFS` reader - Trim for DP Fall Time in FS"]
pub type TfdpfsR = crate::FieldReader;
#[doc = "Field `TFDPFS` writer - Trim for DP Fall Time in FS"]
pub type TfdpfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRDPFS` reader - Trim for DP Rise Time in FS"]
pub type TrdpfsR = crate::FieldReader;
#[doc = "Field `TRDPFS` writer - Trim for DP Rise Time in FS"]
pub type TrdpfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&self) -> RoutR {
        RoutR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&self) -> EndlypullupR {
        EndlypullupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&self) -> DlypullupfsR {
        DlypullupfsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&self) -> VcrsfsR {
        VcrsfsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&self) -> TfdmfsR {
        TfdmfsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&self) -> TrdmfsR {
        TrdmfsR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&self) -> TfdpfsR {
        TfdpfsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&self) -> TrdpfsR {
        TrdpfsR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&mut self) -> RoutW<'_, Dattrim1Spec> {
        RoutW::new(self, 0)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&mut self) -> EndlypullupW<'_, Dattrim1Spec> {
        EndlypullupW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&mut self) -> DlypullupfsW<'_, Dattrim1Spec> {
        DlypullupfsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&mut self) -> VcrsfsW<'_, Dattrim1Spec> {
        VcrsfsW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&mut self) -> TfdmfsW<'_, Dattrim1Spec> {
        TfdmfsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&mut self) -> TrdmfsW<'_, Dattrim1Spec> {
        TrdmfsW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&mut self) -> TfdpfsW<'_, Dattrim1Spec> {
        TfdpfsW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&mut self) -> TrdpfsW<'_, Dattrim1Spec> {
        TrdpfsW::new(self, 18)
    }
}
#[doc = "Data TRIM 1 Values for USB DP and DM\n\nYou can [`read`](crate::Reg::read) this register and get [`dattrim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dattrim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dattrim1Spec;
impl crate::RegisterSpec for Dattrim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dattrim1::R`](R) reader structure"]
impl crate::Readable for Dattrim1Spec {}
#[doc = "`write(|w| ..)` method takes [`dattrim1::W`](W) writer structure"]
impl crate::Writable for Dattrim1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATTRIM1 to value 0x24"]
impl crate::Resettable for Dattrim1Spec {
    const RESET_VALUE: u32 = 0x24;
}
