#[doc = "Register `MIR0_ARB` reader"]
pub type R = crate::R<Mir0ArbSpec>;
#[doc = "Register `MIR0_ARB` writer"]
pub type W = crate::W<Mir0ArbSpec>;
#[doc = "Field `ID` reader - Message Identifier"]
pub type IdR = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - Message Identifier"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `DIR` reader - Message Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Message Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTD` reader - Extended Identifier"]
pub type XtdR = crate::BitReader;
#[doc = "Field `XTD` writer - Extended Identifier"]
pub type XtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSGVAL` reader - Message Valid"]
pub type MsgvalR = crate::BitReader;
#[doc = "Field `MSGVAL` writer - Message Valid"]
pub type MsgvalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XtdR {
        XtdR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MsgvalR {
        MsgvalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<'_, Mir0ArbSpec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, Mir0ArbSpec> {
        DirW::new(self, 29)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&mut self) -> XtdW<'_, Mir0ArbSpec> {
        XtdW::new(self, 30)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&mut self) -> MsgvalW<'_, Mir0ArbSpec> {
        MsgvalW::new(self, 31)
    }
}
#[doc = "Interface Arbitration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_arb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_arb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir0ArbSpec;
impl crate::RegisterSpec for Mir0ArbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_arb::R`](R) reader structure"]
impl crate::Readable for Mir0ArbSpec {}
#[doc = "`write(|w| ..)` method takes [`mir0_arb::W`](W) writer structure"]
impl crate::Writable for Mir0ArbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR0_ARB to value 0"]
impl crate::Resettable for Mir0ArbSpec {}
