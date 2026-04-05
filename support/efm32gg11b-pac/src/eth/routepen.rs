#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `MDIOPEN` reader - MDIO I/O Enable"]
pub type MdiopenR = crate::BitReader;
#[doc = "Field `MDIOPEN` writer - MDIO I/O Enable"]
pub type MdiopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIITXERPEN` reader - MII TX ER I/O Enable"]
pub type MiitxerpenR = crate::BitReader;
#[doc = "Field `MIITXERPEN` writer - MII TX ER I/O Enable"]
pub type MiitxerpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIIRXERPEN` reader - MII TX ER I/O Enable"]
pub type MiirxerpenR = crate::BitReader;
#[doc = "Field `MIIRXERPEN` writer - MII TX ER I/O Enable"]
pub type MiirxerpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIIPEN` reader - MII I/O Enable"]
pub type MiipenR = crate::BitReader;
#[doc = "Field `MIIPEN` writer - MII I/O Enable"]
pub type MiipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMIIPEN` reader - RMII I/O Enable"]
pub type RmiipenR = crate::BitReader;
#[doc = "Field `RMIIPEN` writer - RMII I/O Enable"]
pub type RmiipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUTMRTOGPEN` reader - TSU_TMR_CNT_SEC Output Enable"]
pub type TsutmrtogpenR = crate::BitReader;
#[doc = "Field `TSUTMRTOGPEN` writer - TSU_TMR_CNT_SEC Output Enable"]
pub type TsutmrtogpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&self) -> MdiopenR {
        MdiopenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&self) -> MiitxerpenR {
        MiitxerpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&self) -> MiirxerpenR {
        MiirxerpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&self) -> MiipenR {
        MiipenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&self) -> RmiipenR {
        RmiipenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&self) -> TsutmrtogpenR {
        TsutmrtogpenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&mut self) -> MdiopenW<'_, RoutepenSpec> {
        MdiopenW::new(self, 0)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&mut self) -> MiitxerpenW<'_, RoutepenSpec> {
        MiitxerpenW::new(self, 1)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&mut self) -> MiirxerpenW<'_, RoutepenSpec> {
        MiirxerpenW::new(self, 2)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&mut self) -> MiipenW<'_, RoutepenSpec> {
        MiipenW::new(self, 3)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&mut self) -> RmiipenW<'_, RoutepenSpec> {
        RmiipenW::new(self, 4)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&mut self) -> TsutmrtogpenW<'_, RoutepenSpec> {
        TsutmrtogpenW::new(self, 5)
    }
}
#[doc = "I/O Route Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoutepenSpec;
impl crate::RegisterSpec for RoutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for RoutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for RoutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for RoutepenSpec {}
