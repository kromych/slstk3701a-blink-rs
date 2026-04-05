#[doc = "Register `STACKEDVLAN` reader"]
pub type R = crate::R<StackedvlanSpec>;
#[doc = "Register `STACKEDVLAN` writer"]
pub type W = crate::W<StackedvlanSpec>;
#[doc = "Field `MATCH` reader - User defined VLAN_TYPE field"]
pub type MatchR = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - User defined VLAN_TYPE field"]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENBPROCESSING` reader - Enable stacked VLAN processing mode"]
pub type EnbprocessingR = crate::BitReader;
#[doc = "Field `ENBPROCESSING` writer - Enable stacked VLAN processing mode"]
pub type EnbprocessingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&self) -> EnbprocessingR {
        EnbprocessingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&mut self) -> MatchW<'_, StackedvlanSpec> {
        MatchW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&mut self) -> EnbprocessingW<'_, StackedvlanSpec> {
        EnbprocessingW::new(self, 31)
    }
}
#[doc = "Stacked VLAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stackedvlan::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stackedvlan::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StackedvlanSpec;
impl crate::RegisterSpec for StackedvlanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stackedvlan::R`](R) reader structure"]
impl crate::Readable for StackedvlanSpec {}
#[doc = "`write(|w| ..)` method takes [`stackedvlan::W`](W) writer structure"]
impl crate::Writable for StackedvlanSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STACKEDVLAN to value 0"]
impl crate::Resettable for StackedvlanSpec {}
